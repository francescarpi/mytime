# mytime

This project was created for an educational purpose. Just to learn rust :).

It's used to tracking the working time.


## Usage

Run `mytime` to show the available commands.

Show the table tasks:

```bash
mytime show
mytime show -r today
mytime show --range today
mytime show --range week
mytime show --range month
```

Start a new task:

```bash
mytime start --desc "My task"
mytime start -d "My task"
```

Stop the current active task:

```bash
mytime stop
```

Modify the description of a task:

```bash
mytime modify --id 14 --desc "New description"
mytime modify -i 14 -d "New description"
```

Reopen a closed task:

```bash
mytime reopen --id 12
mytime reopen -i 12
```

## Database

The first time `mytime` is executed, it creates the folder `$HOME/.local/share/mytime`. It contains a `sqlite3` database (`mytime.db`) to store data.
