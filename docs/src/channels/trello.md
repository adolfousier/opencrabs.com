# Trello

OpenCrabs integrates with Trello for board and card management via the `trello_send` tool.

## Setup

1. Get an API Key and Token from [trello.com/power-ups/admin](https://trello.com/power-ups/admin)
2. Configure in `keys.toml`:

```toml
# keys.toml
[channels.trello]
api_key = "your-api-key"
token = "your-token"
```

3. Configure boards and access:

```toml
# config.toml
[channels.trello]
enabled = true
boards = ["Board Name or ID"]
member_id = "your-member-id"
# poll_interval_secs = 300  # Optional: poll for @mentions
```

## Tool Actions

The `trello_send` tool supports 22 actions:

| Action | Description |
|--------|-------------|
| `create_card` | Create a new card |
| `get_card` | Get card details |
| `update_card` | Update card fields |
| `move_card` | Move card to another list |
| `archive_card` | Archive a card |
| `find_cards` | Search for cards |
| `add_comment` | Add a comment to a card |
| `get_card_comments` | Read card comments |
| `add_checklist` | Add a checklist to a card |
| `add_checklist_item` | Add an item to a checklist |
| `complete_checklist_item` | Mark checklist item done |
| `add_label_to_card` | Add a label |
| `remove_label_from_card` | Remove a label |
| `add_member_to_card` | Assign a member |
| `remove_member_from_card` | Unassign a member |
| `add_attachment` | Attach a file or URL |
| `list_boards` | List accessible boards |
| `list_lists` | List columns in a board |
| `get_board_members` | Get board members |
| `search` | Search across boards |
| `get_notifications` | Get notifications |
| `mark_notifications_read` | Mark notifications read |

## Behavior

- **Tool-only by default** — The agent acts on Trello only when explicitly asked
- **Optional polling** — Set `poll_interval_secs` to enable monitoring for `@bot_username` mentions
- **Image attachments** — Generated images are sent as card attachments with embedded previews
- **File attachments** — Card attachments (images, documents) are fetched and processed through the vision pipeline
