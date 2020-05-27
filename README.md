# SLACK-STARLOG

Publishes a message on slack with your daily update.

## Initial setup

Create an incoming slack hook [by following the documentation](https://api.slack.com/messaging/webhooks).
Note the hook URL at the end.

## Configuration
Set the following environment variables:

* `STARLOG_HOOK`: The slack hook URL
* `STARLOG_USERNAME`: The bot's username
* `STARLOG_CHANNEL`: The channel to post the update into

## Usage
Run the application in command-line, follow the instructions.
