use crate::{types::{*}, get_ctx};

pub fn nick_cmd(ctx: SharedClientList, client: SharedClient, args: Vec<&str>) {
    let old_name = get_ctx!(client).name.clone();

    if args.len() != 2 {
        get_ctx!(client).send_server("Usage: /nick <new name>".to_string(), true);
        return;
    }

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

    if args.len() < 3 {
        get_ctx!(client).send_server("Usage: /privmsg <user> <message>".to_string(), true);
        return;
    }

    let target_user = args[0].to_string().clone();
    let mut target_color: u8 = 255;
    let message = format!("\u{001b}[3m\u{001b}[38;5;245m{}\u{001b}[0m\n\r", args[1..].join(" ")) ;
    
    let mut found = false;

    // Find target user
    for c in get_ctx!(ctx).iter() {
        if !get_ctx!(c).name.eq(&target_user.clone()) {
            continue;
        }
        found = true;
        target_color = get_ctx!(c).color.clone();

        // Send message to target
        get_ctx!(c).send_msg(src_name.clone(), src_color.clone(), message.clone());
    }

    if found {
        // Send message to self
        get_ctx!(client).send_self_priv(target_user.clone(), target_color.clone(), message.clone());
    } else {
        // Send message to self
        get_ctx!(client).send_server(format!("User \u{001b}[1m{}\u{001b}[0m not found", target_user.clone()), true);
    }

}

pub fn color_cmd(client: SharedClient, args: Vec<&str>) {
    if args.len() != 2 {
        get_ctx!(client).send_server("Usage: /color <0-255>".to_string(), true);
        return;
    }

    match args[0].to_string().clone().parse::<u8>() {
        Ok(val) => {
            let client_name = get_ctx!(client).name.clone();
            get_ctx!(client).color = val;
            
            // Send a message to you
            get_ctx!(client).send_server(format!("You will now be displayed as \u{001b}[1m\u{001b}[38;5;{}m{}\u{001b}[0m ", val.clone(), client_name.clone()), true);
        },
        Err(_) => {
            get_ctx!(client).send_server("Usage: /color <0-255>".to_string(), true);
            return;
        }
    }    
}

pub fn list_cmd(ctx: SharedClientList, client: SharedClient, reqcmd: bool) {
    let mut client_list = String::new();
    for c in get_ctx!(ctx).iter() {
        let name = get_ctx!(c).name.clone();
        let color = get_ctx!(c).color.clone();

        client_list.push_str(&format!("\u{001b}[1m\u{001b}[38;5;{}m{}\u{001b}[0m ", color.clone(), name.clone()));
    }

    // Send a message to you
    get_ctx!(client).send_server(format!("Connected users: {}", client_list), reqcmd);
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