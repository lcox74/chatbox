use crate::{types::{*}, get_ctx};

pub fn nick_cmd(ctx: SharedClientList, client: SharedClient, args: Vec<&str>) {
    let old_name = get_ctx!(client).name.clone();
    let new_name = args[0].to_string().clone();
    get_ctx!(client).name = new_name.clone();
    
    // Notify everyon else
    for c in get_ctx!(ctx).iter() {
        // Don't send to self
        if get_ctx!(c).name.eq(&new_name.clone()) {
            continue;
        }

        // Send message
        get_ctx!(c).send_server(format!("\u{001b}[1m{}\u{001b}[0m has changed their name to \u{001b}[1m{}\u{001b}[0m", old_name, new_name.clone()), false);
    }

    // Send a message to you
    get_ctx!(client).send_server(format!("Changed name from \u{001b}[1m{}\u{001b}[0m to \u{001b}[1m{}\u{001b}[0m", old_name, new_name.clone()), true);
}

pub fn privmsg_cmd(ctx: SharedClientList, client: SharedClient, args: Vec<&str>) {
    let src_name = get_ctx!(client).name.clone();
    let src_color = get_ctx!(client).color.clone();
    let target_user = args[0].to_string().clone();
    let mut target_color: u8 = 255;
    let message = format!("\u{001b}[38;5;245m{}\u{001b}[0m\n\r", args[1..].join(" ")) ;
    
    // Find target user
    for c in get_ctx!(ctx).iter() {
        if !get_ctx!(c).name.eq(&target_user.clone()) {
            continue;
        }

        target_color = get_ctx!(c).color.clone();

        // Send message to target
        get_ctx!(c).send_msg(src_name.clone(), src_color.clone(), message.clone());
    }

    // Send message to self
    get_ctx!(client).send_self_priv(target_user.clone(), target_color.clone(), message.clone());
}

pub fn color_cmd(client: SharedClient, args: Vec<&str>) {
    let new_color = args[0].to_string().clone().parse::<u8>().unwrap();
    let client_name = get_ctx!(client).name.clone();
    get_ctx!(client).color = new_color;
    
    // Send a message to you
    get_ctx!(client).send_server(format!("You will now be displayed as \u{001b}[1m\u{001b}[38;5;{}m{}\u{001b}[0m ", new_color.clone(), client_name.clone()), true);
}

pub fn list_cmd(ctx: SharedClientList, client: SharedClient) {
    let mut client_list = String::new();
    for c in get_ctx!(ctx).iter() {
        let name = get_ctx!(c).name.clone();
        let color = get_ctx!(c).color.clone();

        client_list.push_str(&format!("\u{001b}[1m\u{001b}[38;5;{}m{}\u{001b}[0m ", color.clone(), name.clone()));
    }

    // Send a message to you
    get_ctx!(client).send_server(format!("Connected users: {}", client_list), true);
}

pub fn send_join_msg(ctx: SharedClientList, client_name: String) {
    for c in get_ctx!(ctx).iter() {
        // Don't send to self
        if get_ctx!(c).name.eq(&client_name) {
            continue;
        }

        // Send message
        get_ctx!(c).send_server(format!("\u{001b}[1m{}\u{001b}[0m has joined the chat", client_name.clone()), false);
    }
}

pub fn send_normal_msg(ctx: SharedClientList, client: SharedClient, message: String) {
    let src_name = get_ctx!(client).name.clone();
    let src_color = get_ctx!(client).color.clone();
    
    // Send message to all clients
    for c in get_ctx!(ctx).iter() {
        // Don't send to self
        if get_ctx!(c).name.eq(&src_name.clone()) {
            continue;
        }
        
        // Send message
        get_ctx!(c).send_msg(src_name.clone(), src_color.clone(), message.clone());
    }
    
    // Send message to self
    get_ctx!(client).send_self(message.clone());
}

pub fn send_leave_msg(ctx: SharedClientList, client_name: String, client_color: u8) {
    for c in get_ctx!(ctx).iter() {
        // Don't send to self
        if get_ctx!(c).name.eq(&client_name.clone()) {
            continue;
        }

        // Send message
        get_ctx!(c).send_server(format!("\u{001b}[1m\u{001b}[38;5;{}m{}\u{001b}[0m has left the chat\n\r", client_color.clone(), client_name.clone()), false);
    }
}