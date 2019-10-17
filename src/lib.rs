pub mod proto {
    mod google {
        mod protobuf {
            tonic::include_proto!("google.protobuf");
        }
        mod api {
            tonic::include_proto!("google.api");
        }
    }

    mod gogoproto {
        tonic::include_proto!("gogoproto");
    }

    mod authpb {
        tonic::include_proto!("authpb");
    }

    mod mvccpb {
        tonic::include_proto!("mvccpb");
    }

    pub mod etcdserverpb {
        tonic::include_proto!("etcdserverpb");
    }

    pub mod electionpb {
        pub use crate::proto::etcdserverpb::{
            AlarmRequest, AlarmResponse, AuthDisableRequest, AuthDisableResponse,
            AuthEnableRequest, AuthEnableResponse, AuthRoleAddRequest, AuthRoleAddResponse,
            AuthRoleDeleteRequest, AuthRoleDeleteResponse, AuthRoleGetRequest, AuthRoleGetResponse,
            AuthRoleGrantPermissionRequest, AuthRoleGrantPermissionResponse, AuthRoleListRequest,
            AuthRoleListResponse, AuthRoleRevokePermissionRequest,
            AuthRoleRevokePermissionResponse, AuthUserAddRequest, AuthUserAddResponse,
            AuthUserChangePasswordRequest, AuthUserChangePasswordResponse, AuthUserDeleteRequest,
            AuthUserDeleteResponse, AuthUserGetRequest, AuthUserGetResponse,
            AuthUserGrantRoleRequest, AuthUserGrantRoleResponse, AuthUserListRequest,
            AuthUserListResponse, AuthUserRevokeRoleRequest, AuthUserRevokeRoleResponse,
            AuthenticateRequest, AuthenticateResponse, CompactionRequest, CompactionResponse,
            DefragmentRequest, DefragmentResponse, DeleteRangeRequest, DeleteRangeResponse,
            HashKvRequest, HashKvResponse, HashRequest, HashResponse, LeaseGrantRequest,
            LeaseGrantResponse, LeaseKeepAliveRequest, LeaseKeepAliveResponse, LeaseLeasesRequest,
            LeaseLeasesResponse, LeaseRevokeRequest, LeaseRevokeResponse, LeaseTimeToLiveRequest,
            LeaseTimeToLiveResponse, MemberAddRequest, MemberAddResponse, MemberListRequest,
            MemberListResponse, MemberPromoteRequest, MemberPromoteResponse, MemberRemoveRequest,
            MemberRemoveResponse, MemberUpdateRequest, MemberUpdateResponse, MoveLeaderRequest,
            MoveLeaderResponse, PutRequest, PutResponse, RangeRequest, RangeResponse,
            SnapshotRequest, SnapshotResponse, StatusRequest, StatusResponse, TxnRequest,
            TxnResponse, WatchRequest, WatchResponse,
        };

        tonic::include_proto!("v3electionpb");
    }

    pub mod lockpb {
        pub use crate::proto::electionpb::{
            CampaignRequest, CampaignResponse, LeaderRequest, LeaderResponse, ProclaimRequest,
            ProclaimResponse, ResignRequest, ResignResponse,
        };

        pub use crate::proto::etcdserverpb::{
            AlarmRequest, AlarmResponse, AuthDisableRequest, AuthDisableResponse,
            AuthEnableRequest, AuthEnableResponse, AuthRoleAddRequest, AuthRoleAddResponse,
            AuthRoleDeleteRequest, AuthRoleDeleteResponse, AuthRoleGetRequest, AuthRoleGetResponse,
            AuthRoleGrantPermissionRequest, AuthRoleGrantPermissionResponse, AuthRoleListRequest,
            AuthRoleListResponse, AuthRoleRevokePermissionRequest,
            AuthRoleRevokePermissionResponse, AuthUserAddRequest, AuthUserAddResponse,
            AuthUserChangePasswordRequest, AuthUserChangePasswordResponse, AuthUserDeleteRequest,
            AuthUserDeleteResponse, AuthUserGetRequest, AuthUserGetResponse,
            AuthUserGrantRoleRequest, AuthUserGrantRoleResponse, AuthUserListRequest,
            AuthUserListResponse, AuthUserRevokeRoleRequest, AuthUserRevokeRoleResponse,
            AuthenticateRequest, AuthenticateResponse, CompactionRequest, CompactionResponse,
            DefragmentRequest, DefragmentResponse, DeleteRangeRequest, DeleteRangeResponse,
            HashKvRequest, HashKvResponse, HashRequest, HashResponse, LeaseGrantRequest,
            LeaseGrantResponse, LeaseKeepAliveRequest, LeaseKeepAliveResponse, LeaseLeasesRequest,
            LeaseLeasesResponse, LeaseRevokeRequest, LeaseRevokeResponse, LeaseTimeToLiveRequest,
            LeaseTimeToLiveResponse, MemberAddRequest, MemberAddResponse, MemberListRequest,
            MemberListResponse, MemberPromoteRequest, MemberPromoteResponse, MemberRemoveRequest,
            MemberRemoveResponse, MemberUpdateRequest, MemberUpdateResponse, MoveLeaderRequest,
            MoveLeaderResponse, PutRequest, PutResponse, RangeRequest, RangeResponse,
            SnapshotRequest, SnapshotResponse, StatusRequest, StatusResponse, TxnRequest,
            TxnResponse, WatchRequest, WatchResponse,
        };

        tonic::include_proto!("v3lockpb");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
