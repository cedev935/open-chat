import type { Identity } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { idlFactory, UserService } from "./candid/idl";
import type {
    EventsResponse,
    UpdateArgs,
    CandidateGroupChat,
    CreateGroupResponse,
    DirectChatEvent,
    MergedUpdatesResponse,
    ChatSummary,
} from "../../domain/chat/chat";
import { CandidService } from "../candidService";
import {
    chunkResponse,
    createGroupResponse,
    getEventsResponse,
    getUpdatesResponse,
} from "./mappers";
import type { IUserClient } from "./user.client.interface";
import type { ChunkResponse } from "../../domain/data/data";
import { mergeChatUpdates } from "../../domain/chat/chat.utils";

export class UserClient extends CandidService implements IUserClient {
    private userService: UserService;

    constructor(identity: Identity, userId: Principal) {
        super(identity);
        this.userService = this.createServiceClient<UserService>(idlFactory, userId.toString());
    }

    createGroup(group: CandidateGroupChat): Promise<CreateGroupResponse> {
        return this.handleResponse(
            this.userService.create_group({
                is_public: group.isPublic,
                name: group.name,
            }),
            createGroupResponse
        );
    }

    chatEvents(
        userId: string,
        fromIndex: number,
        toIndex: number
    ): Promise<EventsResponse<DirectChatEvent>> {
        return this.handleResponse(
            this.userService.events({
                user_id: Principal.fromText(userId),
                to_index: toIndex,
                from_index: fromIndex,
            }),
            getEventsResponse
        );
    }

    async getUpdates(
        chatSummaries: ChatSummary[],
        args: UpdateArgs
    ): Promise<MergedUpdatesResponse> {
        const updatesResponse = await this.handleResponse(
            this.userService.updates({
                updates_since: args.updatesSince
                    ? [
                          {
                              timestamp: args.updatesSince.timestamp,
                              group_chats: args.updatesSince.groupChats.map((g) => ({
                                  chat_id: Principal.fromText(g.chatId),
                                  updates_since: g.lastUpdated,
                              })),
                          },
                      ]
                    : [],
            }),
            (resp) => getUpdatesResponse(resp)
        );
        return {
            chatSummaries: mergeChatUpdates(chatSummaries, updatesResponse),
            timestamp: updatesResponse.timestamp,
        };
    }

    async getData(blobId: bigint, totalBytes?: number, chunkSize?: number): Promise<ChunkResponse> {
        if (!totalBytes || !chunkSize) {
            return this.getChunk(blobId, 0);
        }
        return undefined;
    }

    private async getChunk(blobId: bigint, chunkIndex: number): Promise<ChunkResponse> {
        return this.handleResponse(
            this.userService.chunk({
                blob_id: blobId,
                index: chunkIndex,
            }),
            chunkResponse
        );
    }
}
