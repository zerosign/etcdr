.PHONY: prepare

clean-proto:
	rm -rf proto/*

# manual updates by referencing to master
# (TODO: @zerosign, able to fetch by tag or just use git submodule instead)
prepare: clean-proto
	mkdir -p proto/{google/{protobuf,api},gogoproto,etcd/etcdserver/etcdserverpb,etcd/mvcc/mvccpb,etcd/v3lock/v3lockpb,etcd/v3election/v3electionpb,etcd/auth/authpb}
	curl -Lo proto/etcd/auth/authpb/auth.proto https://raw.githubusercontent.com/etcd-io/etcd/master/auth/authpb/auth.proto
	curl -Lo proto/google/protobuf/descriptor.proto	https://raw.githubusercontent.com/protocolbuffers/protobuf/master/src/google/protobuf/descriptor.proto
	curl -Lo proto/google/api/annotations.proto https://raw.githubusercontent.com/googleapis/googleapis/master/google/api/annotations.proto
	curl -Lo proto/google/api/http.proto https://raw.githubusercontent.com/googleapis/googleapis/master/google/api/http.proto
	curl -Lo proto/gogoproto/gogo.proto https://raw.githubusercontent.com/gogo/protobuf/master/gogoproto/gogo.proto
	curl -Lo proto/etcd/etcdserver/etcdserverpb/rpc.proto https://raw.githubusercontent.com/etcd-io/etcd/master/etcdserver/etcdserverpb/rpc.proto
	curl -Lo proto/etcd/mvcc/mvccpb/kv.proto https://raw.githubusercontent.com/etcd-io/etcd/master/mvcc/mvccpb/kv.proto
	curl -Lo proto/etcd/v3lock/v3lockpb/v3lock.proto https://raw.githubusercontent.com/etcd-io/etcd/master/etcdserver/api/v3lock/v3lockpb/v3lock.proto
	curl -Lo proto/etcd/v3election/v3electionpb/v3election.proto https://raw.githubusercontent.com/etcd-io/etcd/master/etcdserver/api/v3election/v3electionpb/v3election.proto
