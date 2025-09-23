use crate::examples_error::ExamplesError;
use couchbase::cluster::Cluster;
use couchbase::management::users::user::{Role, User};

pub async fn create_user(cluster: Cluster) -> Result<(), ExamplesError> {
    // #tag::upsertuser[]
    let mgr = cluster.users();
    mgr.upsert_user(
        User::new("example-user", "display-name", vec![Role::new("admin")]).password("password"),
        None,
    )
    .await?;
    // #end::upsertuser[]

    Ok(())
}

pub async fn get_all_users(cluster: Cluster) -> Result<(), ExamplesError> {
    // #tag::getusers[]
    let mgr = cluster.users();
    let users = mgr.get_all_users(None).await?;
    // #end::getusers[]

    Ok(())
}

pub async fn remove_user(cluster: Cluster) -> Result<(), ExamplesError> {
    // #tag::removeuser[]
    let mgr = cluster.users();
    mgr.drop_user("example-user", None).await?;
    // #end::removeuser[]

    Ok(())
}

pub async fn change_password(cluster: Cluster) -> Result<(), ExamplesError> {
    // #tag::changepassword[]
    let mgr = cluster.users();
    mgr.change_password("new-password", None).await?;
    // #end::changepassword[]

    Ok(())
}
