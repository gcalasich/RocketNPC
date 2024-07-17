use chrono::DateTime;
use chrono::Utc;
use rocket::serde::{Deserialize, Serialize};
use std::fmt;
//use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub enum APIErrorTypes {
    GeneralException = -2,
    ExceptionBLS = -1,
    Success = 0,
    ErrorBLSAPI = 1,
    MissingAPIName = 2,
    WrongAPIRequestParams = 3,
    MissingAuthentication = 4,
    UserNoRightsForAPI = 5,
    AuthenticationError = 6,
    MissingMandatoryParam = 7,
    WrongParams = 8,
    APINotImplemented = 9,
    BPError = 10,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIError {
    pub error_type: APIErrorTypes,
    pub error_message: String,
    pub file: String,
    pub method: String,
    pub error_datestamp: DateTime<Utc>,
    pub error_info: String,
    pub error_code: APIErrorCodes,
}

impl APIError {
    pub fn new(
        error_type: APIErrorTypes,
        error_message: String,
        file: String,
        method: String,
        error_datestamp: DateTime<Utc>,
        error_info: String,
        error_code: APIErrorCodes,
    ) -> APIError {
        APIError {
            error_type,
            error_message,
            file,
            method,
            error_datestamp,
            error_info,
            error_code,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum APIErrorCodes {
    MAIINV01, // Main Component - Invoke Method - 01 - Error Decoding API Request JSON
    MAIINV02, // Main Component - Invoke Method - 02 - Error Getting Pool Connection from R2D2
    MAIINV03, // Main Component - Invoke Method - 03 - Error Deserializing API Request JSON
    MAIINV04, // Main Component - Invoke Method - 04 - Error Deserializing API Request JSON
    MAIINV05, // Main Component - Invoke Method - 05 - Expired Token
    MAIINV06, // Main Component - Invoke Method - 06 - Error Deserializing API Request JSON - Start Transaction
    MAIINV07, // Main Component - Invoke Method - 07 - Error Deserializing API Request JSON - Commit
    MAIINV08, // Main Component - Invoke Method - 08 - Error Deserializing API Request JSON - Rollback
    MAIINV09, // Main Component - Invoke Method - 09 - Error Deserializing API Request JSON - GETMARKETLIST
    MAIINV10, // Main Component - Invoke Method - 10 - Error Deserializing API Request JSON - GETMARKETBYID
    MAIINV11, // Main Component - Invoke Method - 11 - Error Deserializing API Request JSON - CREATE MARKET
    MAIINV12, // Main Component - Invoke Method - 11 - Error Deserializing API Request JSON - UPDATE MARKET
    MAIVTO01, // Main Component - validate_token_with_transaction - 01 - Token Expired
    MAIVTO02, // Main Component - validate_token - 02 - Token Expired

    APPAUTAUT01, // Application Section - Autenticate Component - Authenticate Method  - 01 -  User Not Found
    APPAUTAUT02, // Application Section - Autenticate Component - Authenticate Method - 02 -  General DB exception
    APPAUTAUT03, // Application Section - Autenticate Component - Authenticate Method - 03 -  Failed Hashing Password
    APPAUTAUT04, // Application Section - Autenticate Component - Authenticate Method - 04 -  Failed Validating Hashed Password
    APPAUTAUT05, // Application Section - Autenticate Component - Authenticate Method - 04 -  Invalid Password

    APPVTOVTO01, //Application Section - Validate Token Component  - Validate Token Method - Failed to add Session TimeOut to last Authentication time.
    APPVTOVTO02, //Application Section - Validate Token Component  - Validate Token Method - Failed Validating Token
    APPVTOVTO03, //Application Section - Validate Token Component  - Validate Token Method - Token expired
    APPVTOVTO04, //Application Section - Validate Token Component  - Validate Token With Transaction Method - Failed Validating Token
    APPVTOVTO05, //Application Section - Validate Token Component  - Validate Token With Transaction Method - Token expired

    APPCOMGPI01, //Application Section - Commit - Get Pool Id - No TransactionId, UserId found in TransactionPoolInUse

    APPMARGML01, //Application Section - Market - Get Market List - Error Serializing Market Vector JSON
    APPMARGMI01, //Application Section - Market - Get Market By Id - Error Serializing Market JSON
    APPMARGMI02, //Application Section - Market - Get Market By Id - Error Serializing Get Market By ID JSON

    APPMARCMA01, //Application Section - Market - Create Market Method - Error Serializing Create Market JSON
    APPMARUMA01, //Application Section - Market - Update Market Method - Error Serializing Update Market JSON

    APPMARDMA01, //Application Section - Market - Delete Market Method - Error Serializing Delete Market JSON

    APPPROGPL01, //Application Section - Product - Get Product List - Error Serializing Product Vector JSON
    APPPROGPI01, //Application Section - Product - Get Product By Id - Error Serializing Product JSON
    APPPROGPI02, //Application Section - Product - Get Product By Id - Error Serializing Get Porduct by Id JSON

    APPPROCPR01, //Application Section - Product - Create Product - Error Serializing Create Product JSON
    APPPROUPR01, //Application Section - Product - Update Product - Error Serializing Update Product JSON

    APPPRODPR01, //Application Section - Product - Delete Product - Error Serializing Delete Product JSON

    APPPROGPSL01, //Application Section - Product - Get Product Service List - Error Serializing Get Product Service List JSON

    APPPROGPSL02, //Application Section - Product - Get Product Service List - Error Serializing  Product Service Vector JSON

    APPPROCPS01, //Application Section - Product - Create Product Service - Error Serializing Create Product Service JSON

    APPPROUPS01, //Application Section - Product - Update Product Service - Error Serializing Update Product Service JSON

    APPPRODPS01, //Application Section - Product - Delete Product Service - Error Serializing Delete Product Service JSON

    APPSERGSL01, //Application Section - Service - Get Service List - Error Serializing Get Service List JSON
    APPSERGSL02, //Application Section - Service - Get Service List - Error Serializing Service Vector JSON

    APPSERGSI01, //Application Section - Service - Get Service By Id - Error Serializing Service JSON
    APPSERGSI02, //Application Section - Service - Get Service By Id - Error Serializing Get Porduct by Id JSON
    APPSERUSE01, //Application Section - Service - Update Service - Error Serializing Update Service JSON
    APPSERCSE01, //Application Section - Service - Create Service - Error Serializing Create Service JSON

    APPSERDSE01, //Application Section - Service - Delete Service - Error Serializing Delete Service JSON

    APPSERGSTI01, //Application Section - Service - Get Service Type by Id - Error Serializing Get Service Type By ID JSON
    APPSERGSTI02, //Application Section - Service - Get Service Type By Id - Error Serializing service_type JSON
    APPSERDST01, //Application Section - Service - Delete Service Type - Error Serializing Delete Service Type JSON
    APPSTYUST01, //Application Section - Service - Update Service Type - Error Serializing update service type  JSON
    APPMARGMP01, //Application Section - Market - Get Market Product By Id - Error Serializing Get Market Product By Id JSON
    APPMARGMP02, //Application Section - Market - Get Market Product By Id - Error Serializing Market Product JSON

    APPMARGPML01, //Application Section - Market - Get Market Product List - Error Serializing Get Products By Market Id JSON
    APPMARGPML02, //Application Section - Market - Get Market Product List - Cannot pass product Id and market id at the same time
    APPMARGPML03, //Application Section - Market - Get Market Product List - Error Serializing Market Products JSON

    APPMARCMP01, //Application Section - Market - Create Market Product - Error Serializing Create Market Product JSON

    APPMARUMP01, //Application Section - Market - Update Market Product - Error Serializing Update Market Product JSON

    APPMARDMP01, //Application Section - Market - Delete Market Product - Error Serializing Delete Market Product JSON
    APPUSRGUI01, //Application Section  - User - Get User By Id - Error Serializing Get User By Id JSON
    APPUSRGUI02, //Application Section  - User - Get User By Id - Error Serializing User JSON

    APPUSRGUL01, //Application Section  - User - Get User List - Error Serializing Get User List JSON
    APPUSRGUL02, //Application Section  - User - Get User List - Error Serializing Users Vec JSON

    APPUSRUUS01, //Application Section  - User - Update User - Error Serializing Update User JSON
    APPUSRUUS02, //Application Section  - User - Update User - Error Hashing Password
    APPUSRDEL01, //Application Section  - User - Delete User - Error Serializing Delete User JSON

    APPUSRCRU01, //Application Section - User - Create User - Error Serializing Create User JSON
    APPUSRCRU02, //Application Section - User - Create User - Error Hashing Password

    APPUSRCRR01, //Application Section - Role - Create Role - Error Serializing Create Role JSON

    APPROLUPR01, //Application Section - Role - Update Role - Error Serializing Update Role JSON

    APPROLGPRI01, //Application Section - Role - Get Permissions By Role Id - Error Serializing Get Permissions By Role Id JSON
    APPROLGPRI02, //Application Section - Role - Get Permissions By Role Id - Error Serializing Permissions By Role Id Vector JSON

    APPROLGRPI01, //Application Section - Role - Get Permissions By Role Id - Error Serializing Get Role Permission By Id JSON
    APPROLGRPI02, //Application Section - Role - Get Permissions By Role Id - Error Serializing Role Permission JSON

    APPROLCRP01, //Application Section - Role - Create Role Permission - Error Serializing Create Role Permission JSON
    APPROLURP01, //Application Section - Role - Update Role Permission - Error Serializing Update Role Permission JSON
    APPROLDRP01, //Application Section - Role - Delete Role Permission - Error Serializing Delete Role Permission JSON

    APPROLDRO01, //Application Section - Role - Delete Role - Error Serializing Delte Role JSON

    APPRESGRL01, //Application Section  - Resource - Get Resource List - Error Serializing Get Resource List JSON
    APPRESGRL02, //Application Section  - Resource - Get Resource List - Error Serializing Resource Vec JSON

    APPRESGRI01, //Application Section  - Resource - Get Resource By Id - Error Serializing Get Resource By Id JSON
    APPRESGRI02, //Application Section  - Resource - Get Resource By ID - Error Serializing Resource JSON

    APPRESCRR01, //Application Section  - Resource - Create Resource - Error Serializing Create Resource JSON

    APPRTYGRTL01, //Application Section  - Resource Type - Get Resource Type By Id - Error Serializing Get Resource Type By Id JSON
    APPRTYGRTL02, //Application Section  - Resource Type - Get Resource Type By Id - Error Serializing Resource Type JSON

    APPRTYGRTI01, //Application Section  - Resource Type - Get Resource Type List - Error Serializing Resource Type Vec JSON
    APPRTYGRTI02, //Application Section  - Resource Type - Get Resource Type List - Error Serializing Resource Type Vec JSON

    APPRTYURT01, //Application Section  - Resource Type - Update Resource Type - Error Serializing Update Resource Type JSON
    APPRTYDRT01, //Application Section  - Resource Type - Delete Resource Type - Error Serializing Delete Resource Type JSON

    INFSECJWTETO01, // Infrastructure Section - Security Component - JWT Helper - Encode Token - Secret Env Key Required
    INFSECJWTETO02, //  Infrastructure Section - Security Component - JWT Helper - Encode Token - Failed Encoding Token
    INFSECJWTETO03, //  Infrastructure Section - Security Component - JWT Helper - Encode Token - Failed Decoding Token
    INFDBHGPI01, //  Infrastructure Section - DB Component - DB Helper - Get Pool Id - No TransactionId, UserId found in TransactionPoolInUse
    INFDBHGPI02, // Infrastructure Section - DB Component - DB Helper - Get Transaction Pool Id - All Transaction Pools Are in Use
    INFDBHGPI03, // Infrastructure Section - DB Component - DB Helper - Get Connection By Pool Id - Failed Retriving Connection from Transaction Pool
    INFDBHGPI04, // Infrastructure Section - DB Component - DB Helper - Get Pool Connection - Failed Retriving Connection from DB Pool
    INFDBHFTP01, // Infrastructure Section - DB Component - DB Helper - Free Transaction Pool - Error Freeing Transction Pool from Transaction Pool Vex
    INFDBHHDE01, // Infrastructure Section - DB Component - DB Helper - Handle Internal Diesel Error - No Records Found with this Criteria
    INFDBHHDE02, // Infrastructure Section - DB Component - DB Helper - Handle Internal Diesel Error - Postgres DB Diesel Error
    INFDBHHDE03, // Infrastructure Section - DB Component - DB Helper - Handle Internal Diesel Error - Postgres Other Error
    INFDBHGCI01, // Infrastructure Section - DB Component - DB Helper - Get Connection Id - Cannot get connection Id from connection
    INFDBHVTC01, // Infrastructure Section - DB Component - DB Helper - Validate Transaction Connection - Transaction Id and Connection Id do not match
    APPCUSCCU01, //Application Section  - Customer - Create Customer - Error Serializing Create Customer Type JSON
    APPCUSGCL01, //Application Section  - Customer - Get Customer List - Error Serializing Get Customer List Type JSON
    APPCUSGCL02, //Application Section  - Customer - Get Customer List - Error Serializing Get Customer List JSON
    APPCUSGCU01, //Application Section  - Customer - Get Customer - Error Serializing Get Customer Type JSON
    APPCUSGCU02, //Application Section  - Customer - Get Customer - Error CustomerId and Identifier missing
    APPCUSGCU03, //Application Section  - Customer - Get Customer - Error Serializing Get Customer JSON
    APPCUSSCS01, //Application Section  - Customer - Set Customer Status- Error Serializing Set Customer Status  Type JSON
    APPCUSGCI01, //Application Section  - Customer - Get Customer By Id - Error Serializing Get Customer By Id JSON
    APPCUSGCI02, //Application Section  - Customer - Get Customer By Id - Error Serializing Customer DB to JSON
    APPCUSCCUS01, //Application Section - Customer - Create Customer Status - Error Serializing Create Customer Status JSON
    APPCUSUCU01, //Application Section  - Customer - Update Customer - Error Serializing Update Customer JSON
    APPCUSDCU01, //Application Section  - Customer - Delete Customer - Error Serializing Delete Customer JSON
    APPSUBCSU01, //Application Section  - Subscriber - Create Subscriber - Error Serializing Create Subscriber JSON
    APPSUBCSU02, //Application Section  - Subscriber - Create Subscriber - Customer Id is Required
    APPSUBUSU01, //Application Section  - Subscriber -Update Subscriber - Error Serializing Update Subscriber JSON
    APPSUBSSU01, //Application Section  - Subscriber - Suspend Subscriber - Error Serializing Suspend Subscriber JSON
    APPSUBUSS01, //Application Section  - Subscriber - UnSuspend Subscriber - Error Serializing UnSuspend Subscriber JSON
    APPSUBGSL01, //Application Section  - Subscriber - Get Subscriber List - Error Serializing Get Subscriber List JSON
    APPSUBTSU01, //Application Section  - Subscriber - Terminate Subscriber - Error Serializing Terminate Subscriber JSON
    APPSUBGSI01, //Application Section  - Subscriber - Get Subscriber By Id - Error Serializing Get Subscriber By Id JSON
    APPSUBGSI02, //Application Section  - Subscriber - Get Subscriber By Id - Error Serializing Subscriber DB to JSON
    APPSUBDSU01, //Application Section  - Subscriber - Delete Subscriber - Error Serializing Delete Subscriber JSON
    APPSUBGSPL01, //Application Section  - Subscriber - Get Subscriber Product List - Error Serializing Get Subscriber Product List JSON
    APPSUBGSPL02, //Application Section  - Subscriber - Get Subscriber Product List - Error Cannot pass Subscriber Id and product id at the same time
    APPSUBGSPL03, //Application Section  - Subscriber - Get Subscriber Product List - Error Serializing Subscriber Product Vector JSON
    APPSUBGSPI01, //Application Section  - Subscriber - Get Subscriber Product By Id - Error Serializing Get Subscriber Product By Id JSON
    APPSUBGSPI02, //Application Section  - Subscriber - Get Subscriber Product By Id - Error Serializing Subscriber Product JSON
    APPSUBCSP01, //Application Section  - Subscriber - Create Subscriber Product - Error Serializing Create Subscriber Product JSON
    APPSUBCSP02, //Application Section  - Subscriber - Create Subscriber Product - Subscriber Id and Product Id are Required.
    APPSUBUSP01, //Application Section  - Subscriber - Update Subscriber Product - Error Serializing Update Subscriber Product JSON
    APPSUBDSP01, //Application Section  - Subscriber - Delete Subscriber Product - Error Serializing Delete Subscriber Product JSON
    APPSUBGSSL01, //Application Section  - Subscriber - Get Subscriber Service List - Error Serializing Get Subscriber Service List JSON
    APPSUBGSSL02, //Application Section  - Subscriber - Get Subscriber Service List - Cannot pass subscriber Id and service id at the same time
    APPSUBGSSL03, //Application Section  - Subscriber - Get Subscriber Service List - Error Serializing Subscriber Servcice Vector JSON
    APPSUBGSSI01, //Application Section  - Subscriber - Get Subscriber Service By Id - Error Serializing Get Subscriber Service By Id JSON
    APPSUBGSSI02, //Application Section  - Subscriber - Get Subscriber Service By Id - Error Serializing Subscriber Service JSON
    APPSUBCSS01, //Application Section  - Subscriber - Create Subscriber Service - Error Serializing Subscriber Service JSON
    APPSUBCSS02, //Application Section  - Subscriber - Create Subscriber Service - Subscriber Id and Service Id are Required.
    APPSUBUPSS01, //Application Section  - Subscriber - Update Subscriber Service - Error Serializing Update Subscriber Service JSON
    APPSUBDSS01, //Application Section  - Subscriber - Delete Subscriber Service - Error Serializing Delete Subscriber Service JSON
    APPSUBGSBL01, //Application Section  - Subscriber - Get Subscriber Bank List - Error Serializing Get Subscriber Bank List JSON
    APPSUBGSBL02, //Application Section  - Subscriber - Get Subscriber Bank List - Cannot pass subscriber Id and bank id at the same time
    APPSUBGSBL03, //Application Section  - Subscriber - Get Subscriber Bank List - Error Serializing Subscriber Bank Vector JSON
    APPSUBGSBI01, //Application Section  - Subscriber - Get Subscriber Bank By Id - Error Serializing Get Subscriber Bank By Id JSON
    APPSUBGSBI02, //Application Section  - Subscriber - Get Subscriber Bank By Id - Error Serializing Subscriber Bank JSON
    APPSUBCSB01, //Application Section  - Subscriber - Create Subscriber Bank - Error Serializing Subscriber Bank JSON
    APPSUBCSB02, //Application Section  - Subscriber - Create Subscriber Bank - Subscriber Id and Bank Id are Required.
    APPSUBUSB01, //Application Section  - Subscriber - Update Subscriber Bank - Error Serializing Update Subscriber Bank JSON
    APPSUBDSB01, //Application Section  - Subscriber - Delete Subscriber Bank - Error Serializing Delete Subscriber Bank JSON
    APPSUBGSAL01, //Application Section  - Subscriber - Get Subscriber Account List - Error Serializing Get Subscriber Account List JSON
    APPSUBGSAL02, //Application Section  - Subscriber - Get Subscriber Account List - Cannot pass subscriber Id and account id at the same time
    APPSUBGSAL03, //Application Section  - Subscriber - Get Subscriber Account List - Error Serializing Subscriber Account Vector JSON
    APPSUBGSABI01, //Application Section  - Subscriber - Get Subscriber Account By Id - Error Serializing Get Subscriber Account By Id JSON
    APPSUBGSABI02, //Application Section  - Subscriber - Get Subscriber Account By Id - Error Serializing Subscriber Account JSON
    APPSUBCSA01, //Application Section  - Subscriber - Create Subscriber Account - Error Serializing Subscriber Account JSON
    APPSUBCSA02, //Application Section  - Subscriber - Create Subscriber Account - Subscriber Id and ICCID are Required.
    APPSUBUSA01, //Application Section  - Subscriber - Update Subscriber Account - Error Serializing Update Subscriber Account JSON
    APPSUBDSA01, //Application Section  - Subscriber - Delete Subscriber Account - Error Serializing Delete Subscriber Account JSON
    APPHELCOMGUD01, //Application Section  - Helper - Common - Get Date UTC - Error Converting Date to Date UTC
    APPHELCOMGUDON01, //Application Section  - Helper - Common - Get Date UTC OR NOW - Error Converting Date to Date UTC
    APPHELCOMGNO02, //Application Section  - Helper - Common - Get Number Option - Error Converting String to Number i64
    APPHELCOMGIO03, //Application Section  - Helper - Common - Get Integer Option - Error Converting String to Number i32
    APPHELCOMGIO04, //Application Section  - Helper - Common - Get Date Option - Error Converting String to Naive Date
    APPCUSUCI01, //Application Section  - Customer - Update Customer Identification - Error Serializing Update Customer Identification JSON
    APPBANGBL01, //Application Section  - Bank - Get Bank List - Error Serializing Get Bank List JSON
    APPBANGBL02, //Application Section - Bank - Get Bank List - Error Serializing Bank List Vector JSON
    APPBANGBI01, //Application Section  - Bank - Get Bank By Id - Error Serializing Get Bank By Id JSON
    APPBANGBI02, //Application Section - Bank - Get Bank By Id - Error Serializing Bank JSON
    APPBANCBA01, //Application Section  - Bank - Create Bank - Error Serializing Create Bank JSON
    APPBANUBA01, //Application Section  - Bank - Update Bank - Error Serializing Update Bank JSON
    APPBANDBA01, //Application Section  - Bank - Delete Bank - Error Serializing Delete Bank JSON
    APPBANGBTL01, //Application Section  - Bank - Get Bank Type List - Error Serializing Get Bank Type List JSON
    APPBANGBTL02, //Application Section - Bank - Get Bank Type List - Error Serializing Bank Type List Vector JSON
    APPBANCBT01, //Application Section  - Bank - Update Bank Type  - Error Serializing Update Bank Type  JSON
    APPBANUBTY01, //Application Section  - Bank - Create Bank Type  - Error Serializing Create Bank Type  JSON
    APPMARGMBL01, //Application Section  - Market - Get Market Bank List - Error Serializing Get Market Bank List JSON
    APPMARGMBL02, //Application Section - Market - Get Market Bank List - Error Serializing Market Bank List Vector JSON
    APPMARCMB01, //Application Section  - Market - Create Market Bank - Error Serializing Create Market Bank JSON
    APPMARUMB01, //Application Section  - Market - Update Market Bank - Error Serializing Update Market Bank JSON
    APPMARGMBI01, //Application Section  - Market - Get Market Bank By - Error Serializing Get Market Bank By Id JSON
    APPMARDMB01, //Application Section  - Market - Delete Market Bank - Error Serializing Delete Market Bank JSON
    APPPROGPBL01, //Application Section  - Product - Get Product Bank List - Error Serializing Get Product Bank List JSON
    APPPROGPBL02, //Application Section  - Product - Get Product Bank List - Cannot pass bank Id and product id at the same time
    APPPROGPBL03, //Application Section  - Product - Get Product Bank List - Error Serializing Product Bank Vector JSON
    APPPROGPDL01, //Application Section  - Product - Get Product Detail List - Error Serializing Get Product Detail List JSON
    APPPROGPDL02, //Application Section  - Product - Get Product Detail List - Cannot pass service Id and product id at the same time
    APPPROGPDL03, //Application Section  - Product - Get Product Detail List - Error Serializing Product Detail Vector JSON
    APPPRODGPDI01, //Application Section  - Product - Get Product Detail By Id - Error Serializing Get Product Detail By Id JSON
    APPPRODGPDI02, //Application Section  - Product - Get Product Detail By Id - Error Serializing Product Detail JSON
    APPPROCPD01, //Application Section  - Product - Create Product Detail - Error Serializing Create Product Detail JSON
    APPPROUPD01, //Application Section  - Product - Update Product Detail - Error Serializing Update Product Detail  JSON
    APPPROCPB01, //Application Section  - Product - Create Product Bank - Error Serializing Create Product Bank JSON
    APPPRODGPBI01, //Application Section  - Bank - Get Product Bank By Id - Error Serializing Get Product Bank By Id JSON
    APPPRODGPBI02, //Application Section  - Bank - Get Bank By Id - Error Serializing Product Bank JSON
    APPSERGSBL01, //Application Section  - Service - Get Service Bank List - Error Serializing Get Service Bank List JSON
    APPSERGSBL02, //Application Section  - Service - Get Service Bank List - Cannot pass bank Id and service id at the same time
    APPSERGSBL03, //Application Section  - Service - Get Service Bank List - Error Serializing Service Bank Vector JSON
    APPSERGSBI01, //Application Section  - Service - Get Service Bank By Id - Error Serializing Get Service Bank By Id JSON
    APPSERGSBI02, //Application Section  - Service - Get Service Bank By Id - Error Serializing Service Bank JSON
    APPSERUSB01, //Application Section  - Service - Update Service Bank  - Error Serializing Update Service Bank JSON
    APPSERDSB01, //Application Section  - Service - Delete Service Bank  - Error Serializing Delete Service Bank JSON
    APPCUSGCSL01, //Application Section  - Customer - Get Customer Service List - Error Serializing Get Customer Service List JSON
    APPCUSGCSL02, //Application Section  - Customer - Get Customer Service List - Cannot pass service Id and customer id at the same time
    APPCUSGCSL03, //Application Section  - Customer - Get Customer Service List - Error Serializing Customer Servcice Vector JSON
    APPCUSCCS01, //Application Section  - Customer - Create Customer Service - Error Serializing Customer Service JSON
    APPCUSGCSI01, //Application Section  - Customer - Get Customer Service By Id - Error Serializing Get Customer Service By Id JSON
    APPCUSGCSI02, //Application Section  - Customer - Get Customer Service By Id - Error Serializing Customer Service JSON
    APPCUSUCS01, //Application Section  - Customer - Update Customer Service  - Error Serializing Update Customer Service JSON
    APPCUSGCBL01, //Application Section  - Customer - Get Customer Bank List  - Error Serializing Get Customer Bank List JSON
    APPCUSGCBL02, //Application Section  - Customer - Get Customer Bank List  - Cannot pass Bank Id and customer id at the same time
    APPCUSGCBL03, //Application Section  - Customer - Get Customer Bank List  - Error Serializing Customer Bank Vector JSON
    APPCUSGCBI01, //Application Section  - Customer - Get Customer Bank By Id  - Error Serializing Customer Bank By Id JSON
    APPCUSGCBI02, //Application Section  - Customer - Get Customer Bank By Id  - Error Serializing Customer Bank JSON
    APPCUSGCPL01, //Application Section  - Customer - Get Customer Product List - Error Serializing Get Customer Product List JSON
    APPCUSGCPL02, //Application Section  - Customer - Get Customer Product List - Cannot pass product Id and customer id at the same time
    APPCUSGCPL03, //Application Section  - Customer - Get Customer Product List - Error Serializing Customer Product Vector JSON
    APPCUSCCP01, //Application Section  - Customer - Create Customer Product - Error Serializing Customer Product JSON
    APPCUSGCPI01, //Application Section  - Customer - Get Customer Product By Id - Error Serializing Get Customer Product By Id JSON
    APPCUSGCPI02, //Application Section  - Customer - Get Customer Product By Id - Error Serializing Get Customer Product By Id JSON
    APPCUSUCP01, //Application Section  - Customer - Update Customer Product - Error Serializing Update Customer Product  JSON
    APPCUSDCP01, //Application Section  - Customer - Delete Customer Product - Error Serializing Update Delete Product  JSON
    APPCUSGCT01, //Application Section  - Customer - Get Customer Type List  - Error Serializing Get Customer Type List JSON
    APPCUSGCT02, //Application Section  - Customer - Get Customer Type By Id  - Error Serializing Customer Type JSON
    APPCUSCTBI01, //Application Section  - Customer - Get Customer Type By Id - Error Serializing Get Customer Type By Id JSON
    APPCUSCTBI02, //Application Section  - Customer - Get Customer Type By Id  - Error Serializing Customer Type JSON
    APPCUSCCT01, //Application Section  - Customer - Create Customer Type - Error Serializing Create Customer Type JSON
    APPCUSUCT01, //Application Section  - Customer - Update Customer Type - Error Serializing Update Customer Type JSON
    APPCUSDCT01, //Application Section  - Customer - Delete Customer Type - Error Serializing Delete Customer Type JSON
    APPICCGILL01, //Application Section  - OCSICCID - Get ICCID Look Up List - Error Serializing Get ICCID Look Up List JSON
    APPICCGILL02, //Application Section  - OCSICCID - Get ICCID Look Up List - Error Serializing Get ICCID Look Up Vector JSON
    APPRESGREL01, //Application Section  - Resource ESN - Get Resource ESN Look Up List - Error Serializing Get Resource ESN Look Up List JSON
    APPRESGREL02, //Application Section  - Resource ESN - Get Resource ESN Look Up List - Error Serializing Get Resource ESN Look Up Vector JSON
    APPIPNUGPLL01, //Application Section  - Phone Number - Get Phone Number Look Up List - Error Serializing Get Phone Number Look Up List JSON
    APPIPNUGPLL02, //Application Section  - Phone Number - Get Phone Number Look Up List - Error Serializing Get Phone Number Look Up Vector JSON
    APPSERGSCTL01, //Application Section  - Service - Get Service Class Type List  - Error Serializing Get Service Class Type List JSON
    APPSERGSCTL02, //Application Section  - Service - Get Service Class Type List  - Error Serializing Service Class Type Vector JSON
    APPSERGSCTI01, //Application Section  - Service - Get Service Class Type By Id - Error Serializing Get Service Class Type By Id JSON
    APPSERGSCTI02, //Application Section  - Service - Get Service Class Type By Id - Error Serializing Service Class Type JSON
    APPSERCSCT01, //Application Section  - Service - Create Service Class Type - Error Serializing Create Service Class Type JSON
    APPSERUSCT01, //Application Section  - Service - Update Service Class Type  - Error Serializing Update Service Class Type JSON
    APPSERDSCT01, //Application Section  - Service - Delete Service Class Type  - Error Serializing Delete Service Class Type JSON
    APPSERGSCL01, //Application Section  - Service - Get Service Class List  - Error Serializing Get Service Class List JSON
    APPSERGSCL02, //Application Section  - Service - Get Service Class List  - Error Serializing Service Class Vector JSON
    APPSERGSCI01, //Application Section  - Service - Get Service Class By Id - Error Serializing Get Service Class By Id JSON
    APPSERGSCI02, //Application Section  - Service - Get Service Class By Id - Error Serializing Service Class JSON
    APPSERCSC01, //Application Section  - Service - Create Service Class - Error Serializing Create Service Class JSON
    APPSERUSC01, //Application Section  - Service - Update Service Class  - Error Serializing Update Service Class JSON
    APPSERDSC01, //Application Section  - Service - Delete Service Class  - Error Serializing Delete Service Class JSON
    APPSLCGSLC01, //Application Section - SysLangCode - Get SysLangCode List - Error Serializing Get SysLangCode List JSON
    APPSLCGSLC02, //Application Section - SysLangCode - Get SysLangCode List - Error Serializing SysLangCode Vector JSON
    APPSLCGSLCBI01, //Application Section - SysLangCode - Get SysLangCode By Id - Error Serializing Get SysLangCode By Id JSON
    APPSLCGSLCBI02, //Application Section - SysLangCode - Get SysLangCode By Id - Error Serializing SysLangCode JSON
    APPSLCCSLC01, //Application Section - SysLangCode - Create SysLangCode - Error Serializing Create SysLangCode JSON
    APPSLCUSLC01, //Application Section - SysLangCode - Update SysLangCode - Error Serializing Update SysLangCode JSON
    APPSLCDSLC01, //Application Section - SysLangCode - Delete SysLangCode - Error Serializing Delete SysLangCode JSON
    APPSIDTGSIDT01, //Application Section - SysIdDocType - Get SysIdDocType List - Error Serializing Get SysIdDocType List JSON
    APPSIDTGSIDT02, //Application Section - SysIdDocType - Get SysIdDocType List - Error Serializing SysIdDocType Vector JSON
    APPSIDTGSIDTBI01, //Application Section - SysIdDocType - Get SysIdDocType By Id - Error Serializing Get SysIdDocType By Id JSON
    APPSIDTGSIDTBI02, //Application Section - SysIdDocType - Get SysIdDocType By Id - Error Serializing SysIdDocType JSON
    APPSIDTCSIDT01, //Application Section - SysIdDocType - Create SysIdDocType - Error Serializing Create SysIdDocType JSON
    APPSIDTUSIDT01, //Application Section - SysIdDocType - Update SysIdDocType - Error Serializing Update SysIdDocType JSON
    APPSIDTDSIDT01, //Application Section - SysIdDocType - Delete SysIdDocType - Error Serializing Delete SysIdDocType JSON
    APPSCOGSCL01, //Application Section - SysCountry - Get SysCountry List - Error Serializing Get SysCountry List JSON
    APPSCOGSCL02, //Application Section - SysCountry - Get SysCountry List - Error Serializing SysCountry Vector JSON
    APPRESAMR01, //Application Section - Resource - Allocate Main Resource - Error Serializing Allocate Main Resource JSON
    APPRESSRS01, //Application Section - Resource - Set Resource Status - Error Serializing Set Resource Status JSON
}

impl fmt::Display for APIErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
