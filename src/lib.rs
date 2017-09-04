use std::fmt;
use std::mem;

/// A specific commit type
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum CommitType {
    Breaking,
    Feature,
    Bugfix,
    Other,
    Meta,
}

/// Iterator over the different commit types
pub struct CommitTypeIterator {
    current: Option<CommitType>
}

impl CommitType {
    /// Return the first commit type (Breaking)
    pub fn first_variant() -> CommitType { CommitType::Breaking }

    /// Return the last commit variant (Meta)
    pub fn last_variant() -> CommitType { CommitType::Meta }

    /// Given a commit type, return the next commit type
    pub fn next_variant(&self) -> Option<CommitType> {
        match *self {
            CommitType::Breaking => Some(CommitType::Feature),
            CommitType::Feature => Some(CommitType::Bugfix),
            CommitType::Bugfix => Some(CommitType::Other),
            CommitType::Other => Some(CommitType::Meta),
            CommitType::Meta => None,
        }
    }

    /// Given a commit type, return the previous commit type
    pub fn prev_variant(&self) -> Option<CommitType> {
        match *self {
            CommitType::Breaking => None,
            CommitType::Feature => Some(CommitType::Breaking),
            CommitType::Bugfix => Some(CommitType::Feature),
            CommitType::Other => Some(CommitType::Bugfix),
            CommitType::Meta => Some(CommitType::Other),
        }
    }

    /// Return an iterator over all commit types
    pub fn iter_variants() -> CommitTypeIterator {
        CommitTypeIterator { current: Some(CommitType::first_variant()) }
    }

    /// Return the emoji for this commit type
    pub fn emoji(&self) -> &'static str {
        match *self {
            CommitType::Breaking => "💥",
            CommitType::Feature => "🎉",
            CommitType::Bugfix => "🐛",
            CommitType::Other => "🔥",
            CommitType::Meta => "🌹",
        }
    }

    /// Return the description for this commit type
    pub fn description(&self) -> &'static str {
        match *self {
            CommitType::Breaking => "Breaking change (SemVer: Major)",
            CommitType::Feature => "New functionality (SemVer: Minor)",
            CommitType::Bugfix => "Bugfix (Semver: Patch)",
            CommitType::Other => "Cleanup / Performance (SemVer: patch)",
            CommitType::Meta => "Meta (SemVer: No version difference)",
        }
    }
}

impl fmt::Debug for CommitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CommitType {{ {} }}", self.emoji())
    }
}

impl Iterator for CommitTypeIterator {
    type Item = CommitType;

    fn next(&mut self) -> Option<CommitType> {
        match self.current {
            Some(commit_type) => mem::replace(&mut self.current, commit_type.next_variant()),
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.current {
            Some(CommitType::Breaking) => (5, Some(5)),
            Some(CommitType::Feature) => (4, Some(4)),
            Some(CommitType::Bugfix) => (3, Some(3)),
            Some(CommitType::Other) => (2, Some(2)),
            Some(CommitType::Meta) => (1, Some(1)),
            None => (0, Some(0)),
        }
    }
}

impl ExactSizeIterator for CommitTypeIterator {
    fn len(&self) -> usize {
        5
    }
}

#[cfg(test)]
mod tests {
    use super::CommitType;

    #[test]
    fn it_gives_the_first_type() {
        assert_eq!(CommitType::first_variant(), CommitType::Breaking);
    }

    #[test]
    fn it_gives_the_last_type() {
        assert_eq!(CommitType::last_variant(), CommitType::Meta);
    }

    #[test]
    fn it_gives_the_next_type() {
        assert_eq!(CommitType::Breaking.next_variant(), Some(CommitType::Feature));
        assert_eq!(CommitType::Feature.next_variant(), Some(CommitType::Bugfix));
        assert_eq!(CommitType::Bugfix.next_variant(), Some(CommitType::Other));
        assert_eq!(CommitType::Other.next_variant(), Some(CommitType::Meta));
        assert_eq!(CommitType::Meta.next_variant(), None);
    }

    #[test]
    fn it_gives_the_previous_type() {
        assert_eq!(CommitType::Breaking.prev_variant(), None);
        assert_eq!(CommitType::Feature.prev_variant(), Some(CommitType::Breaking));
        assert_eq!(CommitType::Bugfix.prev_variant(), Some(CommitType::Feature));
        assert_eq!(CommitType::Other.prev_variant(), Some(CommitType::Bugfix));
        assert_eq!(CommitType::Meta.prev_variant(), Some(CommitType::Other));
    }

    #[test]
    fn it_gives_an_variant_iterator() {
        let mut iter = CommitType::iter_variants();

        assert_eq!(iter.next(), Some(CommitType::Breaking));
        assert_eq!(iter.next(), Some(CommitType::Feature));
        assert_eq!(iter.next(), Some(CommitType::Bugfix));
        assert_eq!(iter.next(), Some(CommitType::Other));
        assert_eq!(iter.next(), Some(CommitType::Meta));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn it_gives_an_emoji() {
        assert_eq!(CommitType::Breaking.emoji(), "💥");
        assert_eq!(CommitType::Feature.emoji(), "🎉");
        assert_eq!(CommitType::Bugfix.emoji(), "🐛");
        assert_eq!(CommitType::Other.emoji(), "🔥");
        assert_eq!(CommitType::Meta.emoji(), "🌹");
    }

    #[test]
    fn it_gives_a_description() {
        assert_eq!(CommitType::Breaking.description(), "Breaking change (SemVer: Major)");
        assert_eq!(CommitType::Feature.description(), "New functionality (SemVer: Minor)");
        assert_eq!(CommitType::Bugfix.description(), "Bugfix (SemVer: Patch)");
        assert_eq!(CommitType::Other.description(), "Cleanup / Performance (SemVer: Patch)");
        assert_eq!(CommitType::Meta.description(), "Meta (SemVer: No version difference)");
    }
}
