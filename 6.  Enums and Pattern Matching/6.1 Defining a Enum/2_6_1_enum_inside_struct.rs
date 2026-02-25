// Listing 6-1: Storing the data and IpAddrKind variant of an IP address using a struct

fn main() {
	enum IpAddrKind {
		V4,
		V6,
	}

	struct IpAddr {
		kind: IpAddrKind,
		address: String,
	}

	let home = IpAddr {
		kind: IpAddrKind::V4,
		address: String::from("127.0.0.1"),
	};

	let loopback = IpAddr {
		kind: IpAddrKind::V6,
		address: String::from("::1"),
	};
}