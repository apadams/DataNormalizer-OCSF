struct Process {
    cmd_line: Option<String>,
    created_time: Option<i64>,
    xattributes: Option<Object>,
    file: Option<File>,
    integrity: Option<String>,
    integrity_id: Option<i64>,
    lineage: Vec<String>,
    loaded_module: Vec<String>,
    name: Option<String>,
    parent_process: Option<Process>,
    pid: Option<i64>,
    sandbox: Option<String>,
    session: Option<Session>,
    terminated_time: Option<i64>,
    thread_id: Option<i64>,
    uid: Option<String>,
    user: Option<User>,
}

struct Account {}

struct EmailAddress {}

struct LdapPerson {}

struct Organization {}

struct User {
    account: Option<Account>,
    uid_alt: Option<String>,
    domain: Option<String>,
    email_addr: Option<EmailAddress>,
    full_name: Option<String>,
    groups: Vec<Group>,
    ldap_person: Option<LdapPerson>,
    name: Option<String>,
    org: Option<Organization>,
    risk_level: Option<String>,
    risk_lvl_id: Option<i64>,
    risk_score: Option<i64>,
    type_: Option<String>,
    type_id: Option<i64>,
    uid: Option<String>,
    credential_uid: Option<String>,
}

struct Device {}

struct Fingerprint {}

struct File {}

struct Idp {
    name: Option<String>,
    uid: Option<String>,
}

struct Enrichment {}

struct Metadata {}

struct Observables {}

struct Object {}

struct Group {}

struct Policy {
    applied: Option<bool>,
    desc: Option<String>,
    group: Option<Group>,
    name: Option<String>,
    uid: Option<String>,
    version: Option<String>,
}

struct Authorizations {
    decision: Option<String>,
    policy: Option<Policy>,
}

struct Session {
    count: Option<i64>,
    created_time: Option<i64>,
    credential_uid: Option<String>,
    expiration_reason: Option<String>,
    expiration_time: Option<i64>,
    mfa: Option<bool>,
    remote: Option<bool>,
    vpn: Option<bool>,
    issuer: Option<String>,
    terminal: Option<String>,
    uid: Option<String>,
    uid_alt: Option<String>,
    uuid: Option<String>,
}

struct Actor {
    app_uid: Option<String>,
    app_name: Option<String>,
    authorizations: Vec<Authorizations>,
    idp: Option<Idp>,
    process: Option<Process>,
    session: Option<Session>,
    user: Option<User>,
}

struct FileActivity {
    access_mask: Option<i64>,
    activity_name: Option<String>,
    activity_id: i64,
    actor: Actor,
    category_name: Option<String>,
    category_uid: i64,
    class_name: Option<String>,
    class_uid: i64,
    component: String,
    connection_uid: Option<String>,
    count: Option<i64>,
    create_mask: Option<i64>,
    device: Device,
    // TODO: Create function to determine duration and remove struct
    duration: Option<i64>,
    end_time: Option<i64>,
    enrichment: Vec<Enrichment>,
    time: i64,
    file_diff: Option<String>,
    file: File,
    file_result: Option<File>,
    message: Option<String>,
    metadata: Metadata,
    observables: Vec<Observables>,
    raw_data: Option<String>,
    severity: Option<String>,
    severity_id: i64,
    start_time: Option<i64>,
    status: Option<String>,
    status_code: Option<String>,
    status_detail: Option<String>,
    status_id: Option<i64>,
    timezone_offset: Option<i64>,
    type_uid: i64,
    type_name: Option<String>,
    unmapped: Option<Object>,
}

fn main() {}