use crate::{types::{*}, get_ctx};

/// Change the user's name. The new name must be unique. Other users will be
/// notified of the change.
/// 
/// # Arguments
/// 
/// * `ctx` - The client list
/// * `client` - The client that sent the command
/// * `args` - The arguments passed to the command
/// 
/// # Example
/// 
/// ```
/// nick_cmd(ctx, client, vec!["new_name"]);
/// ```
pub fn nick_cmd(ctx: SharedClientList, client: SharedClient, args: Vec<&str>) {
    let old_name = get_ctx!(client).name.clone();

    if args.len() != 1 {
        get_ctx!(client).send_server("Usage: /nick <new name>".to_string(), true);
        return;
    }

    let new_name = args[0].to_string().clone();

    // Check if name is taken
    for c in get_ctx!(ctx).iter() {
        if !get_ctx!(c).name.eq(&new_name.clone()) {
            continue;
        }

        get_ctx!(client).send_server(format!("Name \u{001b}[1m{}\u{001b}[0m is already taken", new_name.clone()), true);
        return;
    }

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

    println!("{} | \u{001b}[1m{}\u{001b}[0m changed their name to \u{001b}[1m{}\u{001b}[0m", crate::types::get_timestamp(), old_name, new_name);

}

/// Send a private message to a user. The arguments are concatenated into a single
/// message after the target user name.
/// 
/// # Arguments
/// 
/// * `ctx` - The client list
/// * `client` - The client that sent the command
/// * `args` - The arguments passed to the command
/// 
/// # Example
/// 
/// ```
/// privmsg_cmd(ctx, client, vec!["user", "message", "here", "and", "here"]);
/// ```
pub fn privmsg_cmd(ctx: SharedClientList, client: SharedClient, args: Vec<&str>) {
    let src_name = get_ctx!(client).name.clone();
    let src_color = get_ctx!(client).color.clone();

    if args.len() < 2 {
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

/// Change the color of a client. This only changes how the client is
/// displayed to other clients. Other clients wont be notified of the
/// change.
/// 
/// The colour is a number between 0 and 255. The number is the ANSI
/// color code.
/// 
/// # Arguments
/// 
/// * `client` - The client that sent the command
/// * `args` - The arguments of the command
/// 
/// # Example
/// 
/// ```
/// color_cmd(client, vec!["255"]);
/// ```
pub fn color_cmd(client: SharedClient, args: Vec<&str>) {
    if args.len() != 1 {
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

/// List all connected clients.
/// 
/// # Arguments
/// 
/// * `ctx` - All the clients in the server
/// * `client` - The client that sent the command
/// * `reqcmd` - If the command was sent by the client
/// 
/// # Example
/// 
/// ```
/// list_cmd(ctx, client, true);
/// ```
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

/// Send a join message to all clients from a client.
/// 
/// # Arguments
/// 
/// * `ctx` - All the clients in the server
/// * `client_name` - The name of the client that joined
/// 
/// # Example
/// 
/// ```
/// send_join_msg(ctx, "John Doe".to_string());
/// ```
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

/// Send a regular message to all clients from a client. The sending client
/// will recieve a copy of the message from "You".
/// 
/// # Arguments
/// 
/// * `ctx` - All the clients in the server
/// * `client` - The client the message is sent from
/// * `message` - Message to send
/// 
/// # Example
/// 
/// ```
/// send_normal_msg(ctx, client, "Hello World!".to_string());
/// ```
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


/// Send a message to all clients when a client leaves
/// 
/// # Arguments
/// 
/// * `ctx` - SharedClientList
/// * `client_name` - Name of client leaving
/// * `client_color` - A reference to the color of the client leaving 
/// 
/// # Example
/// 
/// ```
/// send_leave_msg(ctx, "coxy".to_string(), 50);
/// ```
pub fn send_leave_msg(ctx: SharedClientList, client_name: String, client_color: u8) {
    for c in get_ctx!(ctx).iter() {
        // Don't send to self
        if get_ctx!(c).name.eq(&client_name.clone()) {
            continue;
        }

        // Send message
        get_ctx!(c).send_server(format!("\u{001b}[1m\u{001b}[38;5;{}m{}\u{001b}[0m has left the chat", client_color.clone(), client_name.clone()), false);
    }
}