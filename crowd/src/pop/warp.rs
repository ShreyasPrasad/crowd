use super::{PointOfPrescence, PointOfPrescenceClientHandler};
use warp::{filters::ws::WebSocket, Filter};
 
pub struct WarpPointOfPresence;
pub struct WarpPointOfPresenceClientHandler;

impl PointOfPrescence for WarpPointOfPresence {

    async fn init(&self, options: super::PointOfPrescenceOptions) {
        let pop = warp::path("/")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // This will call our function if the handshake succeeds.
            let handler = Self::generate_client_handler();
            ws.on_upgrade(move |socket| {
                handler.receive_client(socket)
            })
        });

        // Define the PoP's routes
        let routes = pop;
        warp::serve(routes).run(options.addr).await;
    }
    
    fn generate_client_handler() -> WarpPointOfPresenceClientHandler {
        WarpPointOfPresenceClientHandler
    }
    
}

impl PointOfPrescenceClientHandler for WarpPointOfPresenceClientHandler {
    type Connection = WebSocket;

    async fn receive_client(self, conn: Self::Connection) {
        todo!()
    }
}
