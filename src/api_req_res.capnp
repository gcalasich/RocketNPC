@0x9663f4dd604afa35;

interface Api_Req_Res {
    struct ApiRequest {
        api_name @0 :Text;
        api_paramas @0 :Text;
    }

    struct Api_Response {
        message @0 :Text;
    }

    sayHello @0 (request: HelloRequest) -> (reply: HelloReply);
}