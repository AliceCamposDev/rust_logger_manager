use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use http::StatusCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[repr(u8)]
pub enum Level {
    Unset = 0,

    Trace = 1,  // rastreamento extremamente detalhado
    Debug = 2,  // debug técnico
    Info = 3,   // informação normal do sistema
    Notice = 4, // evento relevante mas não problema

    Warning = 5,   // algo inesperado mas recuperável
    Error = 6,     // falha de operação
    Critical = 7,  // falha grave
    Alert = 8,     // requer ação imediata
    Emergency = 9, // sistema inutilizável

    Fatal = 10, // vai derrubar o processo
    Panic = 11, // erro irrecuperável

    Custom(u8), // níveis personalizados de 12 a 255
}


#[derive(Debug, Serialize, Deserialize)]
pub struct BaseLog {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub level: Level,
    pub service: String,
    pub environment: String,
    pub message: String,

    pub correlation_id: Option<Uuid>,
    pub request_id: Option<Uuid>,
    pub span_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
    Connect,
    Trace,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpLog {
    pub base: BaseLog,

    pub method: HttpMethod,
    pub path: String,
    pub status_code: u16,
    pub duration_ms: u64,

    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
}