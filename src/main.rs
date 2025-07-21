use std::collections::HashMap;
use std::fmt::Display;

// -----------------------------
// 📌 Note и NoteManager
// -----------------------------

#[derive(Debug)]
// your code

// -----------------------------
// main()
// -----------------------------

fn main() {
    let mut manager = NoteManager::<&str>::new();

    manager.add_note(Note {
        id: 1,
        title: "Rust",
        content: "Ownership and Borrowing",
        tags: vec!["rust", "memory"],
    });

    let note: Note<&str>;
    let title = String::from("Solana");
    let tag1 = String::from("solana");
    let tag2 = String::from("rust");

    note = Note {
        id: 2,
        title: &title,
        content: "Accounts and Programs",
        tags: vec![&tag1, &tag2],
    };
    manager.add_note(note);


    println!("Notes with tag 'rust':");
    for note in manager.get_by_tag("rust") {
        println!("#{} - {} ({})", note.id, note.title, note.content);
    }
}

// -----------------------------
// Unit-тесты
// -----------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut manager = NoteManager::<&str>::new();

        manager.add_note(Note {
            id: 42,
            title: "Test",
            content: "Hello",
            tags: vec!["dev"],
        });

        manager.add_note(Note {
            id: 1,
            title: "Alpha",
            content: "World",
            tags: vec!["misc"],
        });
    }

    #[test]
    fn test_get_by_tag_found() {
        let mut manager = NoteManager::<&str>::new();

        manager.add_note(Note {
            id: 1,
            title: "Rust",
            content: "Borrow checker",
            tags: vec!["rust", "memory"],
        });

        manager.add_note(Note {
            id: 2,
            title: "Solana",
            content: "Programs",
            tags: vec!["blockchain", "rust"],
        });

        let rust_notes = manager.get_by_tag("rust");
        assert_eq!(rust_notes.len(), 2);
        assert_eq!(rust_notes[1].title, "Rust");
        assert_eq!(rust_notes[0].title, "Solana");
    }

    #[test]
    fn test_get_by_tag_not_found() {
        let mut manager = NoteManager::<&str>::new();

        manager.add_note(Note {
            id: 1,
            title: "Only one note",
            content: "No matching tags",
            tags: vec!["nothing"],
        });

        let result = manager.get_by_tag("not_found");
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_generic_with_string() {
        let mut manager = NoteManager::<String>::new();

        manager.add_note(Note {
            id: 10,
            title: "Generic",
            content: "Owned string".to_string(),
            tags: vec!["string"],
        });

        let notes = manager.get_by_tag("string");
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].content, "Owned string");
    }
}