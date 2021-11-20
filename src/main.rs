use std::io;

use chrono::{DateTime, Utc};
use clap::{crate_version, Parser};
use csv::{ByteRecord, Writer};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

/// A tool to stream the chats of Twitch channels as a CSV.
#[derive(Debug, Parser)]
#[clap(
    version = crate_version!(),
    name = "twitch2csv",
    author = "Kerollmops <renault.cle@gmail.com>",
)]
struct Opts {
    /// The list of channels to stream.
    channels: Vec<String>,
}

#[tokio::main]
async fn main() -> csv::Result<()> {
    let opts = Opts::parse();

    let config = ClientConfig::new_simple(StaticLoginCredentials::anonymous());
    let (mut incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, _>::new(config);

    for channel in opts.channels {
        client.join(channel);
    }

    let stdout = io::stdout();
    let mut writer = Writer::from_writer(stdout);
    let mut record = ByteRecord::new();

    writer
        .write_record(
            &[
                "server-timestamp",
                "channel-login",
                "sender-id",
                "sender-login",
                "sender-name",
                "message-id",
                "message-text",
            ][..],
        )
        .or_else(ignore_broken_pipe)?;

    while let Some(message) = incoming_messages.recv().await {
        if let Ok(msg) = TimedUserMessage::from_private_nessage(message) {
            msg.write_in_record(&mut record);
            writer.write_record(&record).or_else(ignore_broken_pipe)?;
        }
    }

    Ok(())
}

fn ignore_broken_pipe(error: csv::Error) -> csv::Result<()> {
    match error.kind() {
        csv::ErrorKind::Io(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        _otherwise => Err(error.into()),
    }
}

#[derive(Debug)]
struct TimedUserMessage {
    server_timestamp: DateTime<Utc>,
    channel_login: String,
    sender_id: String,
    sender_login: String,
    sender_name: String,
    message_id: String,
    message_text: String,
}

impl TimedUserMessage {
    fn from_private_nessage(msg: ServerMessage) -> Result<TimedUserMessage, ServerMessage> {
        match msg {
            ServerMessage::Privmsg(msg) => Ok(TimedUserMessage {
                server_timestamp: msg.server_timestamp,
                channel_login: msg.channel_login,
                sender_id: msg.sender.id,
                sender_login: msg.sender.login,
                sender_name: msg.sender.name,
                message_id: msg.message_id,
                message_text: msg.message_text,
            }),
            otherwise => Err(otherwise),
        }
    }

    fn write_in_record(&self, record: &mut ByteRecord) {
        let TimedUserMessage {
            server_timestamp,
            channel_login,
            sender_id,
            sender_login,
            sender_name,
            message_id,
            message_text,
        } = self;

        record.clear();
        let timestamp = server_timestamp.timestamp().to_string();
        record.push_field(timestamp.as_str().as_bytes());
        record.push_field(channel_login.as_bytes());
        record.push_field(sender_id.as_bytes());
        record.push_field(sender_login.as_bytes());
        record.push_field(sender_name.as_bytes());
        record.push_field(message_id.as_bytes());
        record.push_field(message_text.as_bytes());
    }
}
