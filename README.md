# tracker

A simple command line program to help keep track of work time, storing data in a simple per-week text file. It is designed for the use case where you have flexible work hours, but wish to keep track that you work a certain number of hours per week – by default 40, divided into five work days.

Run `tracker start` to start working. It might look like this (I use `$` here to represent your shell prompt): 

```bash
$ tracker start
[monday 2024-01-08]
* 08:28-
```

As you later end a shift, run `tracker stop`. 

```bash
$ tracker stop
[monday 2024-01-08]
* 08:28-11:40
```

Use `tracker report` to show your progress. 

```bash
$ tracker report
You have worked 3 h 12 m today.
You have worked 3 h 12 m this week.
Balance: -4 h 48 m
```

The balance tells you that you have 4 hours and 48 minutes left to work this day if you wish in order to be in balance. 

While the normal mode of operation is to use `tracker start` and `tracker stop` to track your shifts, you may find that you sometimes forget to start your shift, or otherwise make an error that you wish to correct. Instead of offering a specific user interface to do such edits, `tracker` lets you open the data file for the current week in your text editor of choice (following the `EDITOR` environment variable) by using `tracker edit`.