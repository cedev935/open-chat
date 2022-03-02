use candid::{CandidType, Principal};
use chat_events::GroupChatEvents;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, HashSet};
use types::{
    EventIndex, FallbackRole, GroupPermissions, Mention, MessageIndex, Participant, Role, TimestampMillis, UserId,
    MAX_RETURNED_MENTIONS,
};

const MAX_PARTICIPANTS_PER_PUBLIC_GROUP: u32 = 100_000;
const MAX_PARTICIPANTS_PER_PRIVATE_GROUP: u32 = 200;

#[derive(CandidType, Serialize, Deserialize, Default)]
pub struct Participants {
    by_principal: HashMap<Principal, ParticipantInternal>,
    user_id_to_principal_map: HashMap<UserId, Principal>,
    blocked: HashSet<UserId>,
    admin_count: u32,
}

impl Participants {
    pub fn new(creator_principal: Principal, creator_user_id: UserId, now: TimestampMillis) -> Participants {
        let participant = ParticipantInternal {
            user_id: creator_user_id,
            date_added: now,
            role: Role::Owner,
            min_visible_event_index: EventIndex::default(),
            min_visible_message_index: MessageIndex::default(),
            notifications_muted: false,
            mentions: Vec::new(),
        };

        Participants {
            by_principal: vec![(creator_principal, participant)].into_iter().collect(),
            user_id_to_principal_map: vec![(creator_user_id, creator_principal)].into_iter().collect(),
            blocked: HashSet::new(),
            admin_count: 0,
        }
    }

    pub fn add(
        &mut self,
        user_id: UserId,
        principal: Principal,
        now: TimestampMillis,
        min_visible_event_index: EventIndex,
        min_visible_message_index: MessageIndex,
        as_super_admin: bool,
    ) -> AddResult {
        if self.blocked.contains(&user_id) {
            AddResult::Blocked
        } else {
            match self.by_principal.entry(principal) {
                Vacant(e) => {
                    let participant = ParticipantInternal {
                        user_id,
                        date_added: now,
                        role: if as_super_admin { Role::SuperAdmin(FallbackRole::Participant) } else { Role::Participant },
                        min_visible_event_index,
                        min_visible_message_index,
                        notifications_muted: false,
                        mentions: Vec::new(),
                    };
                    e.insert(participant.clone());
                    self.user_id_to_principal_map.insert(user_id, principal);
                    AddResult::Success(participant)
                }
                _ => AddResult::AlreadyInGroup,
            }
        }
    }

    pub fn remove(&mut self, user_id: UserId) -> bool {
        match self.user_id_to_principal_map.remove(&user_id) {
            None => false,
            Some(principal) => {
                if let Some(participant) = self.by_principal.remove(&principal) {
                    if participant.role.is_admin() {
                        self.admin_count -= 1;
                    }
                }
                true
            }
        }
    }

    pub fn block(&mut self, user_id: UserId) {
        self.blocked.insert(user_id);
    }

    pub fn unblock(&mut self, user_id: &UserId) -> bool {
        self.blocked.remove(user_id)
    }

    pub fn blocked(&self) -> Vec<UserId> {
        self.blocked.iter().copied().collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ParticipantInternal> {
        self.by_principal.values()
    }

    pub fn get(&self, user_id_or_principal: Principal) -> Option<&ParticipantInternal> {
        let principal = self
            .user_id_to_principal_map
            .get(&user_id_or_principal.into())
            .unwrap_or(&user_id_or_principal);

        self.by_principal.get(principal)
    }

    pub fn get_by_user_id(&self, user_id: &UserId) -> Option<&ParticipantInternal> {
        if let Some(p) = self.user_id_to_principal_map.get(user_id) {
            self.get_by_principal(p)
        } else {
            None
        }
    }

    pub fn get_by_user_id_mut(&mut self, user_id: &UserId) -> Option<&mut ParticipantInternal> {
        if let Some(&p) = self.user_id_to_principal_map.get(user_id) {
            self.get_by_principal_mut(&p)
        } else {
            None
        }
    }

    pub fn get_by_principal(&self, principal: &Principal) -> Option<&ParticipantInternal> {
        self.by_principal.get(principal)
    }

    pub fn get_by_principal_mut(&mut self, principal: &Principal) -> Option<&mut ParticipantInternal> {
        self.by_principal.get_mut(principal)
    }

    pub fn is_blocked(&self, user_id: &UserId) -> bool {
        self.blocked.contains(user_id)
    }

    pub fn users_to_notify(&self, my_user_id: UserId) -> HashSet<UserId> {
        self.by_principal
            .values()
            .filter(|p| p.user_id != my_user_id && !p.notifications_muted)
            .map(|p| p.user_id)
            .collect()
    }

    pub fn user_limit_reached(&self, is_public: bool) -> Option<u32> {
        let user_limit = if is_public { MAX_PARTICIPANTS_PER_PUBLIC_GROUP } else { MAX_PARTICIPANTS_PER_PRIVATE_GROUP };

        if self.by_principal.len() >= user_limit as usize {
            Some(user_limit)
        } else {
            None
        }
    }

    pub fn len(&self) -> u32 {
        self.user_id_to_principal_map.len() as u32
    }

    pub fn change_role(
        &mut self,
        caller: Principal,
        user_id: &UserId,
        new_role: Role,
        permissions: &GroupPermissions,
    ) -> ChangeRoleResult {
        // This function cannot be used to make a user a SuperAdmin
        if matches!(new_role, Role::SuperAdmin(_)) {
            return ChangeRoleResult::Invalid;
        }

        // Is the caller authorized to change the user to this role
        let caller_id = match self.get_by_principal(&caller) {
            Some(p) => {
                if !p.role.can_change_roles(new_role, permissions) {
                    return ChangeRoleResult::NotAuthorized;
                }
                p.user_id
            }
            None => return ChangeRoleResult::CallerNotInGroup,
        };

        let mut admin_count = self.admin_count;

        let member = match self.get_by_user_id_mut(user_id) {
            Some(p) => p,
            None => return ChangeRoleResult::UserNotInGroup,
        };

        // It is not possible to change the role of the owner
        if matches!(member.role, Role::Owner) {
            return ChangeRoleResult::Invalid;
        }

        let prev_role = member.role;

        if prev_role == new_role {
            return ChangeRoleResult::Unchanged;
        }

        let mut prev_owner_id: Option<UserId> = None;

        if let Role::SuperAdmin(fallback) = member.role {
            // Super admins can be "changed" to admins or particpants but that just affects
            // the fallback role for when they cease to be a super admin.
            match new_role {
                Role::Admin => {
                    if matches!(fallback, FallbackRole::Admin) {
                        return ChangeRoleResult::Unchanged;
                    }
                    member.role = Role::SuperAdmin(FallbackRole::Admin);
                    admin_count += 1;
                }
                Role::Participant => {
                    if matches!(fallback, FallbackRole::Participant) {
                        return ChangeRoleResult::Unchanged;
                    }
                    member.role = Role::SuperAdmin(FallbackRole::Participant);
                    admin_count -= 1;
                }
                _ => return ChangeRoleResult::Invalid,
            }
        } else {
            if matches!(member.role, Role::Admin) {
                admin_count -= 1;
            }

            member.role = new_role;
            let new_owner_id = member.user_id;

            if matches!(new_role, Role::Owner) {
                // If the member is becoming the owner then any previous owner becomes an admin
                let curr_owner_id = self
                    .iter()
                    .find(|p| p.role.is_owner() && p.user_id != new_owner_id)
                    .map(|p| p.user_id);
                if let Some(owner_id) = curr_owner_id {
                    if let Some(curr_owner) = self.get_by_user_id_mut(&owner_id) {
                        curr_owner.role = Role::Admin;
                        admin_count += 1;
                        prev_owner_id = Some(owner_id);
                    }
                }
                if prev_owner_id.is_none() {
                    return ChangeRoleResult::Invalid;
                }
            } else if matches!(new_role, Role::Admin) {
                admin_count += 1;
            }
        }

        self.admin_count = admin_count;

        ChangeRoleResult::Success(ChangeRoleSuccessResult {
            caller_id,
            prev_owner_id,
            prev_role,
        })
    }

    pub fn make_super_admin(&mut self, user_id: &UserId) -> MakeSuperAdminResult {
        match self.get_by_user_id_mut(user_id) {
            Some(p) => match p.role {
                Role::SuperAdmin(_) => MakeSuperAdminResult::AlreadySuperAdmin,
                Role::Owner => MakeSuperAdminResult::AlreadyOwner,
                Role::Admin => {
                    p.role = Role::SuperAdmin(FallbackRole::Admin);
                    MakeSuperAdminResult::Success
                }
                Role::Participant => {
                    p.role = Role::SuperAdmin(FallbackRole::Participant);
                    MakeSuperAdminResult::Success
                }
            },
            None => MakeSuperAdminResult::NotInGroup,
        }
    }

    pub fn dismiss_super_admin(&mut self, user_id: &UserId) -> DismissSuperAdminResult {
        match self.get_by_user_id_mut(user_id) {
            Some(p) => {
                if let Role::SuperAdmin(fallback_role) = p.role {
                    p.role = fallback_role.into();
                    DismissSuperAdminResult::Success
                } else {
                    DismissSuperAdminResult::NotSuperAdmin
                }
            }
            None => DismissSuperAdminResult::NotInGroup,
        }
    }

    pub fn admin_count(&self) -> u32 {
        self.admin_count
    }

    pub fn add_mention(&mut self, user_id: &UserId, message_index: MessageIndex) -> bool {
        if let Some(p) = self.get_by_user_id_mut(user_id) {
            if p.mentions.is_empty() || (message_index > *p.mentions.last().unwrap()) {
                p.mentions.push(message_index);
                return true;
            }
        }

        false
    }
}

pub enum AddResult {
    Success(ParticipantInternal),
    AlreadyInGroup,
    Blocked,
}

pub enum ChangeRoleResult {
    Success(ChangeRoleSuccessResult),
    CallerNotInGroup,
    NotAuthorized,
    UserNotInGroup,
    Unchanged,
    Invalid,
}

pub struct ChangeRoleSuccessResult {
    pub caller_id: UserId,
    pub prev_owner_id: Option<UserId>,
    pub prev_role: Role,
}

pub enum MakeSuperAdminResult {
    Success,
    NotInGroup,
    AlreadySuperAdmin,
    AlreadyOwner,
}

pub enum DismissSuperAdminResult {
    Success,
    NotInGroup,
    NotSuperAdmin,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct ParticipantInternal {
    pub user_id: UserId,
    pub date_added: TimestampMillis,
    pub role: Role,
    pub notifications_muted: bool,
    pub mentions: Vec<MessageIndex>,

    min_visible_event_index: EventIndex,
    min_visible_message_index: MessageIndex,
}

impl ParticipantInternal {
    pub fn min_visible_event_index(&self) -> EventIndex {
        if self.role.can_view_full_message_history() {
            EventIndex::default()
        } else {
            self.min_visible_event_index
        }
    }

    pub fn min_visible_message_index(&self) -> MessageIndex {
        if self.role.can_view_full_message_history() {
            MessageIndex::default()
        } else {
            self.min_visible_message_index
        }
    }

    pub fn get_most_recent_mentions(&self, events: &GroupChatEvents) -> Vec<Mention> {
        self.mentions
            .iter()
            .rev()
            .filter_map(|message_index| events.hydrate_mention(message_index))
            .take(MAX_RETURNED_MENTIONS)
            .collect()
    }
}

impl From<ParticipantInternal> for Participant {
    fn from(p: ParticipantInternal) -> Self {
        Participant {
            user_id: p.user_id,
            date_added: p.date_added,
            role: p.role,
        }
    }
}

impl From<&ParticipantInternal> for Participant {
    fn from(p: &ParticipantInternal) -> Self {
        Participant {
            user_id: p.user_id,
            date_added: p.date_added,
            role: p.role,
        }
    }
}