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
        // TODO: @zerosign doesn't work, it seems I need to reexport etcdserverpb as super crates
        //       this has something to do that module can't be extended outside of its first definition
        tonic::include_proto!("v3electionpb");
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
