# Final program requirements

- The program should not operate on files individually but instead have a per user database of decks.
    - The program should let the user import or export individual decks from this database if required
    - The database is nothing but a flat file system version controlled by git.
    - Since the database is a git repo the program should provide a mechanism of sycing it with a remote repo.
- The program should let the user open decks from the DB using a tree view
- The program should maintain an list of most recently used decks and give the user an option to open a deck from this list using the select-view.
- The program should let the user create new decks
- The program should let the user modify existing decks
    - add/remove new questions
    - edit existing questions
    - Change the name of the deck
    - Reset the learning schedule
- The program should replicate the learning algorithm used by Anki.
- The program should have a mode where the user can learn from more than one deck at a time.
- The program should show stats about the progress made by the user over time.
- The program should allow the user to include some free form textual information with each deck. This information may be used to identify things like the original source of the information etc.

# To implement next (may list intermediate requirements)
- Modify create_ui to take a model.
- Load deck from hardcoded file path.
- Create DB folder structure at start up.

# Implemented

# Rejected (with reason)
