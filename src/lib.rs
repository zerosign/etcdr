pub mod proto {
    tonic::include_proto!("etcdserverpb");
    tonic::include_proto!("mvccpb");
    tonic::include_proto!("v3lockpb");
    tonic::include_proto!("v3electionpb");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
