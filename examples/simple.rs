use amoeba::{request::Request, route::Route, server::Server};

fn user(request: Request) {
    let user_id = request.param::<u32>("user_id");
    dbg!(user_id);
}

fn main() {
    let mut server = Server::default();
    server.add_handler(Route::new("/users/{user_id}"), user);
    server.run();
}
