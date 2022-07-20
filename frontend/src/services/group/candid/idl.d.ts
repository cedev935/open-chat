import type { IDL } from "@dfinity/candid";
import {
    _SERVICE,
    Message,
    ReplyContext,
    UserId,
    ReplyContext,
    MessageContent,
    FileContent,
    TextContent,
    ImageContent,
    AudioContent,
    VideoContent,
    TimestampMillis,
    BlobReference,
    EventIndex,
    ChatEventWrapper,
    EventsByIndexArgs,
    EventsByIndexResponse,
    EventsResponse,
    EventsSuccessResult,
    EventsArgs,
    ChatEvent,
    AddParticipantsResponse,
    SendMessageArgs,
    SendMessageResponse,
    EditMessageResponse,
    ChangeRoleResponse,
    RemoveParticipantResponse,
    MarkReadResponse,
    UpdateGroupResponse,
    UpdatePermissionsResponse,
    ToggleReactionResponse,
    DeleteMessagesResponse,
    BlockUserResponse,
    UnblockUserResponse,
    SelectedInitialResponse,
    SelectedUpdatesResponse,
    Participant,
    Role,
    MakePrivateResponse,
    PublicGroupSummary,
    PublicSummaryResponse,
    MessagesByMessageIndexResponse,
    MessageEventWrapper,
    PinMessageResponse,
    UnpinMessageResponse,
    RegisterPollVoteResponse,
    SearchMessagesResponse,
    InviteCodeResponse,
    EnableInviteCodeResponse,
    DisableInviteCodeResponse,
    ResetInviteCodeResponse,
    UpdatePermissionsArgs,
    ThreadPreviewsResponse,
    ThreadPreview,
} from "./types";
export {
    _SERVICE as GroupService,
    SendMessageArgs as ApiSendMessageArgs,
    SendMessageResponse as ApiSendMessageResponse,
    EditMessageResponse as ApiEditMessageResponse,
    Message as ApiMessage,
    UserId as ApiUserId,
    ReplyContext as ApiReplyContext,
    MessageContent as ApiMessageContent,
    FileContent as ApiFileContent,
    TextContent as ApiTextContent,
    ImageContent as ApiImageContent,
    VideoContent as ApiVideoContent,
    AudioContent as ApiAudioContent,
    TimestampMillis as ApiTimestampMillis,
    BlobReference as ApiBlobReference,
    EventIndex as ApiEventIndex,
    ChatEventWrapper as ApiEventWrapper,
    EventsByIndexArgs as ApiEventsByIndexArgs,
    EventsByIndexResponse as ApiEventsByIndexResponse,
    EventsResponse as ApiEventsResponse,
    EventsSuccessResult as ApiEventsSuccessResult,
    EventsArgs as ApiEventsArgs,
    ChatEvent as ApiGroupChatEvent,
    AddParticipantsResponse as ApiAddParticipantsResponse,
    ChangeRoleResponse as ApiChangeRoleResponse,
    RemoveParticipantResponse as ApiRemoveParticipantResponse,
    MarkReadResponse as ApiMarkReadResponse,
    UpdateGroupResponse as ApiUpdateGroupResponse,
    UpdatePermissionsResponse as ApiUpdatePermissionsResponse,
    ToggleReactionResponse as ApiToggleReactionResponse,
    DeleteMessagesResponse as ApiDeleteMessageResponse,
    BlockUserResponse as ApiBlockUserResponse,
    UnblockUserResponse as ApiUnblockUserResponse,
    SelectedInitialResponse as ApiSelectedInitialResponse,
    SelectedUpdatesResponse as ApiSelectedUpdatesResponse,
    Participant as ApiParticipant,
    Role as ApiRole,
    MakePrivateResponse as ApiMakePrivateResponse,
    PublicGroupSummary as ApiPublicGroupSummary,
    PublicSummaryResponse as ApiPublicSummaryResponse,
    MessagesByMessageIndexResponse as ApiMessagesByMessageIndexResponse,
    MessageEventWrapper as ApiMessageEventWrapper,
    PinMessageResponse as ApiPinMessageResponse,
    UnpinMessageResponse as ApiUnpinMessageResponse,
    RegisterPollVoteResponse as ApiRegisterPollVoteResponse,
    SearchMessagesResponse as ApiSearchGroupChatResponse,
    InviteCodeResponse as ApiInviteCodeResponse,
    EnableInviteCodeResponse as ApiEnableInviteCodeResponse,
    DisableInviteCodeResponse as ApiDisableInviteCodeResponse,
    ResetInviteCodeResponse as ApiResetInviteCodeResponse,
    UpdatePermissionsArgs as ApiUpdatePermissionsArgs,
    ThreadPreviewsResponse as ApiThreadPreviewsResponse,
    ThreadPreview as ApiThreadPreview,
};

export const idlFactory: IDL.InterfaceFactory;
