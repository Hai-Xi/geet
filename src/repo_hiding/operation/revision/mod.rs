mod cwd;
use super::branch::{get_head, update_head};
use crate::file_hiding::file_log::{retrieve_object, store_object};
use crate::repo_hiding::data_type::{Commit, CommitMetadata, Hash, Tree};
use cwd::{read_cwd, update_cwd};

// create a new revision with the given metadata
pub fn create_revision(metadata: CommitMetadata) -> Hash {
    // create a new commit object
    let commit_hash = read_cwd();
    let parent_hash = get_head();
    let commit = Commit::new_commit(commit_hash.clone(), parent_hash, metadata);

    // store the commit object
    let serialized = commit.serialize();
    store_object(&serialized).unwrap();

    // update HEAD
    update_head(&commit_hash);
    commit_hash
}

// get the revision with the given hash
pub fn get_revision(commit_hash: &String) -> Commit {
    let serialized = retrieve_object(&commit_hash).unwrap();
    Commit::deserialize(&serialized)
}

// get the parent that the revision is pointing to
pub fn get_parent_revision(commit_hash: &String) -> Option<Commit> {
    let commit = get_revision(commit_hash);
    if let Some(parent_hash) = commit.parent {
        Some(get_revision(&parent_hash))
    } else {
        None
    }
}

// apply the changes from the revision to the working directory
pub fn checkout(commit_hash: &String) {
    let commit = get_revision(commit_hash);
    update_cwd(&commit.id);
}
