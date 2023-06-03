# mytime

This project was created for an educational purpose. Just to learn rust :).

It's used to tracking the working time.


## Usage

Execute `mytime --help` to show available commands.

Each argument has its own short version. For instance:

```
mytime show --period today
mytime show -p today
```

In the following example, we are going to use the long version.

Show the tasks table:

```bash
mytime show
mytime show --period today
mytime show --period week
mytime show --period month

mytime show --relative 1 # (1 == -1 == yesterday)
mytime show --relative 2 # (day before yesterday)

mytime show --date "2023-05-30"
```

Start a new task:

```bash
mytime start --desc "My task" --project "Project 1"
mytime start --desc "My task" --project "Project 1" --external_id "12345"
```

Stop the current active task:

```bash
mytime stop
```

Modify a task:

```bash
mytime modify --id 14 --desc "New description"
mytime modify --id 14 --external_id "12345"
mytime modify --id 14 --project "Project 2"
```

Reopen a closed task:

```bash
mytime reopen --id 12
```

Toggle the reported flag. It means that the task has been reported to your own tracking tool, such as redmine or jira.

```bash
mytime report --id 12
```

Send unreported tasks to redmine:

```bash
mytime send
```

## Database

By default, the `sqlite3` database is created to here: `$HOME/.local/share/mytime`. You can change the default path through the config file.

## Config file

Its an  `ini` format file.

```bash
touch $HOME/.mytime
```

```ini
[general]
db_folder = /Users/foo/Library/CloudStorage/Dropbox/mytime

[redmine]
url = <YOUR_REDMINE_DOMAIN>
token = <YOUR_TOKEN>

```

## Build sources

Download source code:

```
git clone git@github.com:francescarpi/mytime.git
```

Run for debug:

```
cd mytime
cargo run
```

For production:

```
cargo build --release
```

The binary file will be into the `target` folder.


## Screenshots

![Screenshot 1](./screenshots/capture1.png)

![Screenshot 2](./screenshots/capture2.png)

![Screenshot 3](./screenshots/capture3.png)

