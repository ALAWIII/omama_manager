//------------------- tables schema---------------------
DEFINE TABLE IF NOT EXISTS chat TYPE ANY SCHEMAFULL PERMISSIONS NONE ;
DEFINE TABLE IF NOT EXISTS message TYPE ANY SCHEMAFULL PERMISSIONS NONE;

DEFINE TABLE IF NOT EXISTS mess_chat TYPE RELATION IN chat OUT message;

//-------------------chat table schema-------------
DEFINE FIELD IF NOT EXISTS id ON TABLE chat TYPE int;
DEFINE FIELD IF NOT EXISTS name ON TABLE chat TYPE string;
DEFINE FIELD IF NOT EXISTS summary ON TABLE chat TYPE option<string>;

//-------------------message table schema--------------
DEFINE FIELD IF NOT EXISTS id ON TABLE message TYPE int;
DEFINE FIELD IF NOT EXISTS message ON TABLE message TYPE string;
DEFINE FIELD IF NOT EXISTS response ON TABLE message TYPE string;

//------------------- chat table schema---------------
