use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
	peer_id: String,
	decoded_peer_id: String,
	// unencoded_peer_id: String,
	// re_encoded_peer_id: String, 
}

enum Msg {
	SetPeerId(String),
	Bs58Decode,
	// SetUnencodedPeerId(String),
	// Bs58Encode,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
			peer_id: "".to_string(),
			decoded_peer_id: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
			Msg::SetPeerId(value) => self.peer_id = value,
			Msg::Bs58Decode => {
				let temp = bs58::decode(&self.peer_id).into_vec().unwrap();
				self.decoded_peer_id = hex::encode(temp);
			},
			// Msg::SetUnencodedPeerId(value) => self.unencoded_peer_id = value,
			// Msg::Bs58Encode => {
			// 	self.decoded_peer_id = bs58::encode(hex::decode(self.unencoded_peer_id)).to_strng();
			// },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
			<div>
				<h1 class="text-lg m-8">{ "Base58 converter for PeerId" }</h1>
				<br />

				<input type="text"
                	placeholder="input your peer id"
					oninput=self.link.callback(|e: InputData| Msg::SetPeerId(e.value))
					class="w-1/2 block text-gray-500 font-bold md:text-left mb-1 md:mb-0 pr-4 m-8 border border-gray-500"
				/>
				<button
					onclick=self.link.callback(|_| Msg::Bs58Decode)
					class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded m-8"
				>
					{ "Decode" }
				</button>
				<br />
				<p class="w-4/6 block text-gray-500 font-bold md:text-left mb-1 md:mb-0 pr-4 m-8 border border-gray-500">
					{ self.decoded_peer_id.clone() }
				</p>

				// <input type="text"
                // 	placeholder="input your decoded peer id"
				// 	oninput=self.link.callback(|e: InputData| Msg::SetUnencodedPeerId(e.value))
				// 	class="w-1/2 block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4 m-8 border border-gray-500"
				// />
				// <button
				// 	onclick=self.link.callback(|_| Msg::Bs58Encode)
				// 	class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded m-8"
				// >
				// 	{ "Encode" }
				// </button>
				// <br />
				// <p>{ self.re_encoded_peer_id.clone() }</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
