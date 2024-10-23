local config = {
    net = {
        http_port = 3001,
        grpc_port = 50051
    },
    logging = {
        place = false,
        level = "debug",
        folder = "logs",
        filescount = 5,
        stdout = true
    }
}

return config
