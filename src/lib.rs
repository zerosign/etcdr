pub mod proto {
    pub mod google {
        pub mod protobuf {
            tonic::include_proto!("google.protobuf");
        }
        pub mod api {
            tonic::include_proto!("google.api");
        }
    }

    pub mod gogoproto {
        tonic::include_proto!("gogoproto");
    }

    pub mod authpb {
        tonic::include_proto!("authpb");
    }

    pub mod mvccpb {
        tonic::include_proto!("mvccpb");
    }

    pub mod etcdserverpb {
        tonic::include_proto!("etcdserverpb");
    }

    pub mod election {

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

    pub mod lock {
        pub use crate::proto::election::{
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
