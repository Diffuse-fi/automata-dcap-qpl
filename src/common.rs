
/** Possible errors generated by the quote interface. */
// #define SGX_QL_MK_ERROR(x)              (0x0000E000|(x))
// typedef enum _quote3_error_t {
//     SGX_QL_SUCCESS = 0x0000,                                         ///< Success
//     SGX_QL_ERROR_MIN = SGX_QL_MK_ERROR(0x0001),                      ///< Indicate min error to allow better translation.
//     SGX_QL_ERROR_UNEXPECTED = SGX_QL_MK_ERROR(0x0001),               ///< Unexpected error
//     SGX_QL_ERROR_INVALID_PARAMETER = SGX_QL_MK_ERROR(0x0002),        ///< The parameter is incorrect
//     SGX_QL_ERROR_OUT_OF_MEMORY = SGX_QL_MK_ERROR(0x0003),            ///< Not enough memory is available to complete this operation
//     SGX_QL_ERROR_ECDSA_ID_MISMATCH = SGX_QL_MK_ERROR(0x0004),        ///< Expected ECDSA_ID does not match the value stored in the ECDSA Blob
//     SGX_QL_PATHNAME_BUFFER_OVERFLOW_ERROR = SGX_QL_MK_ERROR(0x0005), ///< The ECDSA blob pathname is too large
//     SGX_QL_FILE_ACCESS_ERROR = SGX_QL_MK_ERROR(0x0006),              ///< Error accessing ECDSA blob
//     SGX_QL_ERROR_STORED_KEY = SGX_QL_MK_ERROR(0x0007),               ///< Cached ECDSA key is invalid
//     SGX_QL_ERROR_PUB_KEY_ID_MISMATCH = SGX_QL_MK_ERROR(0x0008),      ///< Cached ECDSA key does not match requested key
//     SGX_QL_ERROR_INVALID_PCE_SIG_SCHEME = SGX_QL_MK_ERROR(0x0009),   ///< PCE use the incorrect signature scheme
//     SGX_QL_ATT_KEY_BLOB_ERROR = SGX_QL_MK_ERROR(0x000a),             ///< There is a problem with the attestation key blob.
//     SGX_QL_UNSUPPORTED_ATT_KEY_ID = SGX_QL_MK_ERROR(0x000b),         ///< Unsupported attestation key ID.
//     SGX_QL_UNSUPPORTED_LOADING_POLICY = SGX_QL_MK_ERROR(0x000c),     ///< Unsupported enclave loading policy.
//     SGX_QL_INTERFACE_UNAVAILABLE = SGX_QL_MK_ERROR(0x000d),          ///< Unable to load the PCE enclave
//     SGX_QL_PLATFORM_LIB_UNAVAILABLE = SGX_QL_MK_ERROR(0x000e),       ///< Unable to find the platform library with the dependent APIs.  Not fatal.
//     SGX_QL_ATT_KEY_NOT_INITIALIZED = SGX_QL_MK_ERROR(0x000f),        ///< The attestation key doesn't exist or has not been certified.
//     SGX_QL_ATT_KEY_CERT_DATA_INVALID = SGX_QL_MK_ERROR(0x0010),      ///< The certification data retrieved from the platform library is invalid.
//     SGX_QL_NO_PLATFORM_CERT_DATA = SGX_QL_MK_ERROR(0x0011),          ///< The platform library doesn't have any platfrom cert data.
//     SGX_QL_OUT_OF_EPC = SGX_QL_MK_ERROR(0x0012),                     ///< Not enough memory in the EPC to load the enclave.
//     SGX_QL_ERROR_REPORT = SGX_QL_MK_ERROR(0x0013),                   ///< There was a problem verifying an SGX REPORT.
//     SGX_QL_ENCLAVE_LOST = SGX_QL_MK_ERROR(0x0014),                   ///< Interfacing to the enclave failed due to a power transition.
//     SGX_QL_INVALID_REPORT = SGX_QL_MK_ERROR(0x0015),                 ///< Error verifying the application enclave's report.
//     SGX_QL_ENCLAVE_LOAD_ERROR = SGX_QL_MK_ERROR(0x0016),             ///< Unable to load the enclaves. Could be due to file I/O error, loading infrastructure error, or non-SGX capable system
//     SGX_QL_UNABLE_TO_GENERATE_QE_REPORT = SGX_QL_MK_ERROR(0x0017),   ///< The QE was unable to generate its own report targeting the application enclave either
//                                                                      ///< because the QE doesn't support this feature there is an enclave compatibility issue.
//                                                                      ///< Please call again with the p_qe_report_info to NULL.
//     SGX_QL_KEY_CERTIFCATION_ERROR = SGX_QL_MK_ERROR(0x0018),         ///< Caused when the provider library returns an invalid TCB (too high).
//     SGX_QL_NETWORK_ERROR = SGX_QL_MK_ERROR(0x0019),                  ///< Network error when retrieving PCK certs
//     SGX_QL_MESSAGE_ERROR = SGX_QL_MK_ERROR(0x001a),                  ///< Message error when retrieving PCK certs
//     SGX_QL_NO_QUOTE_COLLATERAL_DATA = SGX_QL_MK_ERROR(0x001b),       ///< The platform does not have the quote verification collateral data available.
//     SGX_QL_QUOTE_CERTIFICATION_DATA_UNSUPPORTED = SGX_QL_MK_ERROR(0x001c),
//     SGX_QL_QUOTE_FORMAT_UNSUPPORTED = SGX_QL_MK_ERROR(0x001d),
//     SGX_QL_UNABLE_TO_GENERATE_REPORT = SGX_QL_MK_ERROR(0x001e),
//     SGX_QL_QE_REPORT_INVALID_SIGNATURE = SGX_QL_MK_ERROR(0x001f),
//     SGX_QL_QE_REPORT_UNSUPPORTED_FORMAT = SGX_QL_MK_ERROR(0x0020),
//     SGX_QL_PCK_CERT_UNSUPPORTED_FORMAT = SGX_QL_MK_ERROR(0x0021),
//     SGX_QL_PCK_CERT_CHAIN_ERROR = SGX_QL_MK_ERROR(0x0022),
//     SGX_QL_TCBINFO_UNSUPPORTED_FORMAT = SGX_QL_MK_ERROR(0x0023),
//     SGX_QL_TCBINFO_MISMATCH = SGX_QL_MK_ERROR(0x0024),
//     SGX_QL_QEIDENTITY_UNSUPPORTED_FORMAT = SGX_QL_MK_ERROR(0x0025),
//     SGX_QL_QEIDENTITY_MISMATCH = SGX_QL_MK_ERROR(0x0026),
//     SGX_QL_TCB_OUT_OF_DATE = SGX_QL_MK_ERROR(0x0027),
//     SGX_QL_TCB_OUT_OF_DATE_CONFIGURATION_NEEDED = SGX_QL_MK_ERROR(0x0028),      ///< TCB out of date and Configuration needed
//     SGX_QL_SGX_ENCLAVE_IDENTITY_OUT_OF_DATE = SGX_QL_MK_ERROR(0x0029),
//     SGX_QL_SGX_ENCLAVE_REPORT_ISVSVN_OUT_OF_DATE = SGX_QL_MK_ERROR(0x002a),
//     SGX_QL_QE_IDENTITY_OUT_OF_DATE = SGX_QL_MK_ERROR(0x002b),
//     SGX_QL_SGX_TCB_INFO_EXPIRED = SGX_QL_MK_ERROR(0x002c),
//     SGX_QL_SGX_PCK_CERT_CHAIN_EXPIRED = SGX_QL_MK_ERROR(0x002d),
//     SGX_QL_SGX_CRL_EXPIRED = SGX_QL_MK_ERROR(0x002e),
//     SGX_QL_SGX_SIGNING_CERT_CHAIN_EXPIRED = SGX_QL_MK_ERROR(0x002f),
//     SGX_QL_SGX_ENCLAVE_IDENTITY_EXPIRED = SGX_QL_MK_ERROR(0x0030),
//     SGX_QL_PCK_REVOKED = SGX_QL_MK_ERROR(0x0031),
//     SGX_QL_TCB_REVOKED = SGX_QL_MK_ERROR(0x0032),
//     SGX_QL_TCB_CONFIGURATION_NEEDED = SGX_QL_MK_ERROR(0x0033),
//     SGX_QL_UNABLE_TO_GET_COLLATERAL = SGX_QL_MK_ERROR(0x0034),
//     SGX_QL_ERROR_INVALID_PRIVILEGE = SGX_QL_MK_ERROR(0x0035),        ///< No enough privilege to perform the operation
//     SGX_QL_NO_QVE_IDENTITY_DATA = SGX_QL_MK_ERROR(0x0037),           ///< The platform does not have the QVE identity data available.
//     SGX_QL_CRL_UNSUPPORTED_FORMAT = SGX_QL_MK_ERROR(0x0038),
//     SGX_QL_QEIDENTITY_CHAIN_ERROR = SGX_QL_MK_ERROR(0x0039),
//     SGX_QL_TCBINFO_CHAIN_ERROR = SGX_QL_MK_ERROR(0x003a),
//     SGX_QL_ERROR_QVL_QVE_MISMATCH = SGX_QL_MK_ERROR(0x003b),          ///< Supplemental data size and version mismatched between QVL and QvE
//                                                                       ///< Please make sure to use QVL and QvE from same release package
//     SGX_QL_TCB_SW_HARDENING_NEEDED = SGX_QL_MK_ERROR(0x003c),         ///< TCB up to date but SW Hardening needed
//     SGX_QL_TCB_CONFIGURATION_AND_SW_HARDENING_NEEDED = SGX_QL_MK_ERROR(0x003d),        ///< TCB up to date but Configuration and SW Hardening needed
// 
//     SGX_QL_UNSUPPORTED_MODE = SGX_QL_MK_ERROR(0x003e),
// 
//     SGX_QL_NO_DEVICE = SGX_QL_MK_ERROR(0x003f),
//     SGX_QL_SERVICE_UNAVAILABLE = SGX_QL_MK_ERROR(0x0040),
//     SGX_QL_NETWORK_FAILURE = SGX_QL_MK_ERROR(0x0041),
//     SGX_QL_SERVICE_TIMEOUT = SGX_QL_MK_ERROR(0x0042),
//     SGX_QL_ERROR_BUSY = SGX_QL_MK_ERROR(0x0043),
// 
//     SGX_QL_UNKNOWN_MESSAGE_RESPONSE  = SGX_QL_MK_ERROR(0x0044),      /// Unexpected error from the cache service
//     SGX_QL_PERSISTENT_STORAGE_ERROR  = SGX_QL_MK_ERROR(0x0045),      /// Error storing the retrieved cached data in persistent memory
//     SGX_QL_ERROR_MESSAGE_PARSING_ERROR   = SGX_QL_MK_ERROR(0x0046),  /// Message parsing error
//     SGX_QL_PLATFORM_UNKNOWN  = SGX_QL_MK_ERROR(0x0047),              /// Platform was not found in the cache
//     SGX_QL_UNKNOWN_API_VERSION  = SGX_QL_MK_ERROR(0x0048),           /// The current PCS API version configured is unknown
//     SGX_QL_CERTS_UNAVAILABLE  = SGX_QL_MK_ERROR(0x0049),             /// Certificates are not available for this platform
// 
//     SGX_QL_QVEIDENTITY_MISMATCH = SGX_QL_MK_ERROR(0x0050),          ///< QvE Identity is NOT match to Intel signed QvE identity
//     SGX_QL_QVE_OUT_OF_DATE = SGX_QL_MK_ERROR(0x0051),               ///< QvE ISVSVN is smaller than the ISVSVN threshold, or input QvE ISVSVN is too small
//     SGX_QL_PSW_NOT_AVAILABLE = SGX_QL_MK_ERROR(0x0052),             ///< SGX PSW library cannot be loaded, could be due to file I/O error
//     SGX_QL_COLLATERAL_VERSION_NOT_SUPPORTED = SGX_QL_MK_ERROR(0x0053),  ///< SGX quote verification collateral version not supported by QVL/QvE
//     SGX_QL_TDX_MODULE_MISMATCH = SGX_QL_MK_ERROR(0x0060),            ///< TDX SEAM module identity is NOT match to Intel signed TDX SEAM module
// 
//     SGX_QL_QEIDENTITY_NOT_FOUND = SGX_QL_MK_ERROR(0x0061),            ///< QE identity was not found
//     SGX_QL_TCBINFO_NOT_FOUND = SGX_QL_MK_ERROR(0x0062),               ///< TCB Info was not found
//     SGX_QL_INTERNAL_SERVER_ERROR = SGX_QL_MK_ERROR(0x0063),           ///< Internal server error
// 
//     SGX_QL_SUPPLEMENTAL_DATA_VERSION_NOT_SUPPORTED = SGX_QL_MK_ERROR(0x0064),       ///< The supplemental data version is not supported
// 
//     SGX_QL_ROOT_CA_UNTRUSTED = SGX_QL_MK_ERROR(0x0065),              ///< The certificate used to establish SSL session is untrusted
// 
//     SGX_QL_TCB_NOT_SUPPORTED = SGX_QL_MK_ERROR(0x0066),              ///< Current TCB level cannot be found in platform/enclave TCB info
// 
//     SGX_QL_CONFIG_INVALID_JSON = SGX_QL_MK_ERROR(0x0067),            ///< The QPL's config file is in JSON format but has a format error
//     
//     SGX_QL_RESULT_INVALID_SIGNATURE = SGX_QL_MK_ERROR(0x0068),    ///< Invalid signature during quote verification
// 
//     SGX_QL_ERROR_MAX = SGX_QL_MK_ERROR(0x00FF),                      ///< Indicate max error to allow better translation.
// 
// } quote3_error_t;

// generate quote3_error enum
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Quote3Error {
    SgxQlSuccess = 0x0000,
    // SgxQlErrorMin = 0xe001,
    SgxQlErrorUnexpected = 0xe001,
    SgxQlErrorInvalidParameter = 0xe002,
    SgxQlErrorOutOfMemory = 0xe003,
    SgxQlErrorEcdsaIdMismatch = 0xe004,
    SgxQlErrorPathnameBufferOverflowError = 0xe005,
    SgxQlErrorFileAccessError = 0xe006,
    SgxQlErrorStoredKey = 0xe007,
    SgxQlErrorPubKeyIdMismatch = 0xe008,
    SgxQlErrorInvalidPceSigScheme = 0xe009,
    SgxQlErrorAttKeyBlobError = 0xe00a,
    SgxQlErrorUnsupportedAttKeyId = 0xe00b,
    SgxQlErrorUnsupportedLoadingPolicy = 0xe00c,
    SgxQlErrorInterfaceUnavailable = 0xe00d,
    SgxQlErrorPlatformLibUnavailable = 0xe00e,
    SgxQlErrorAttKeyNotInitialized = 0xe00f,
    SgxQlErrorAttKeyCertDataInvalid = 0xe010,
    SgxQlErrorNoPlatformCertData = 0xe011,
    SgxQlErrorOutOfEpc = 0xe012,
    SgxQlErrorReport = 0xe013,
    SgxQlErrorEnclaveLost = 0xe014,
    SgxQlErrorInvalidReport = 0xe015,
    SgxQlErrorEnclaveLoadError = 0xe016,
    SgxQlErrorUnableToGenerateQeReport = 0xe017,
    SgxQlErrorKeyCertifcationError = 0xe018,
    SgxQlErrorNetworkError = 0xe019,
    SgxQlErrorMessageError = 0xe01a,
    SgxQlErrorNoQuoteCollateralData = 0xe01b,
    SgxQlErrorQuoteCertificationDataUnsupported = 0xe01c,
    SgxQlErrorQuoteFormatUnsupported = 0xe01d,
    SgxQlErrorUnableToGenerateReport = 0xe01e,
    SgxQlErrorQeReportInvalidSignature = 0xe01f,
    SgxQlErrorQeReportUnsupportedFormat = 0xe020,
    SgxQlErrorPckCertUnsupportedFormat = 0xe021,
    SgxQlErrorPckCertChainError = 0xe022,
    SgxQlErrorTcbinfoUnsupportedFormat = 0xe023,
    SgxQlErrorTcbinfoMismatch = 0xe024,
    SgxQlErrorQeidentityUnsupportedFormat = 0xe025,
    SgxQlErrorQeidentityMismatch = 0xe026,
    SgxQlErrorTcbOutOfDate = 0xe027,
    SgxQlErrorTcbOutOfDateConfigurationNeeded = 0xe028,
    SgxQlErrorSgxEnclaveIdentityOutOfDate = 0xe029,
    SgxQlErrorSgxEnclaveReportIsvsvnOutOfDate = 0xe02a,
    SgxQlErrorQeIdentityOutOfDate = 0xe02b,
    SgxQlErrorSgxTcbInfoExpired = 0xe02c,
    SgxQlErrorSgxPckCertChainExpired = 0xe02d,
    SgxQlErrorSgxCrlExpired = 0xe02e,
    SgxQlErrorSgxSigningCertChainExpired = 0xe02f,
    SgxQlErrorSgxEnclaveIdentityExpired = 0xe030,
    SgxQlErrorPckRevoked = 0xe031,
    SgxQlErrorTcbRevoked = 0xe032,
    SgxQlErrorTcbConfigurationNeeded = 0xe033,
    SgxQlErrorUnableToGetCollateral = 0xe034,
    SgxQlErrorErrorInvalidPrivilege = 0xe035,
    SgxQlErrorNoQveIdentityData = 0xe037,
    SgxQlErrorCrlUnsupportedFormat = 0xe038,
    SgxQlErrorQeidentityChainError = 0xe039,
    SgxQlErrorTcbinfoChainError = 0xe03a,
    SgxQlErrorErrorQvlQveMismatch = 0xe03b,
    SgxQlErrorTcbSwHardeningNeeded = 0xe03c,
    SgxQlErrorTcbConfigurationAndSwHardeningNeeded = 0xe03d,
    SgxQlErrorUnsupportedMode = 0xe03e,
    SgxQlErrorNoDevice = 0xe03f,
    SgxQlErrorServiceUnavailable = 0xe040,
    SgxQlErrorNetworkFailure = 0xe041,
    SgxQlErrorServiceTimeout = 0xe042,
    SgxQlErrorErrorBusy = 0xe043,
    SgxQlErrorUnknownMessageResponse = 0xe044,
    SgxQlErrorPersistentStorageError = 0xe045,
    SgxQlErrorErrorMessageParsingError = 0xe046,
    SgxQlErrorPlatformUnknown = 0xe047,
    SgxQlErrorUnknownApiVersion = 0xe048,
    SgxQlErrorCertsUnavailable = 0xe049,
    SgxQlErrorQveidentityMismatch = 0xe050,
    SgxQlErrorQveOutOfDate = 0xe051,
    SgxQlErrorPswNotAvailable = 0xe052,
    SgxQlErrorCollateralVersionNotSupported = 0xe053,
    SgxQlErrorTdxModuleMismatch = 0xe060,
    SgxQlErrorQeidentityNotFound = 0xe061,
    SgxQlErrorTcbinfoNotFound = 0xe062,
    SgxQlErrorInternalServerError = 0xe063,
    SgxQlErrorSupplementalDataVersionNotSupported = 0xe064,
    SgxQlErrorRootCaUntrusted = 0xe065,
    SgxQlErrorTcbNotSupported = 0xe066,
    SgxQlErrorConfigInvalidJson = 0xe067,
    SgxQlErrorResultInvalidSignature = 0xe068,
    SgxQlErrorMax = 0xe0ff,
}

// sgx_ql_config_version_t
// /** Contains the valid versions of the sgx_ql_config_t data structure. */
// typedef enum _sgx_ql_config_version_t
// {
//     SGX_QL_CONFIG_VERSION_1 = 1,
// }sgx_ql_config_version_t;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SgxQlConfigVersion {
    SgxQlConfigVersion1 = 1,
}

// sgx_ql_pck_cert_id_t
/** Used to describe the PCK Cert for a platform */
// typedef struct _sgx_ql_pck_cert_id_t
// {
//     uint8_t *p_qe3_id;                     ///< The QE_ID used to identify the platform for PCK Cert Retrieval
//     uint32_t qe3_id_size;                  ///< The Size of hte QE_ID (currenlty 16 bytes)
//     sgx_cpu_svn_t *p_platform_cpu_svn;     ///< Pointer to the platform's raw CPUSVN
//     sgx_isv_svn_t *p_platform_pce_isv_svn; ///< Pointer to the platform's raw PCE ISVSVN
//     uint8_t *p_encrypted_ppid;             ///< Pointer to the encrypted PPID (Optional)
//     uint32_t encrypted_ppid_size;          ///< Size of encrytped PPID.
//     uint8_t crypto_suite;                  ///< Crypto algorithm used to encrypt the PPID
//     uint16_t pce_id;                       ///< Identifies the PCE-Version used to generate the encrypted PPID.
// }sgx_ql_pck_cert_id_t;

// generate sgx_ql_pck_cert_id_t as a c type struct
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SgxQlPckCertId {
    pub p_qe3_id: *mut u8,
    pub qe3_id_size: u32,
    pub p_platform_cpu_svn: *mut SgxCpuSvn,
    pub p_platform_pce_isv_svn: *mut SgxIsvSvn,
    pub p_encrypted_ppid: *mut u8,
    pub encrypted_ppid_size: u32,
    pub crypto_suite: u8,
    pub pce_id: u16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SgxCpuSvn {
    pub cpu_svn: [u8; 16],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SgxIsvSvn {
    pub isv_svn: u16,
}

// sgx_ql_config_t
// typedef struct _sgx_ql_config_t
// {
//     sgx_ql_config_version_t version;
//     sgx_cpu_svn_t cert_cpu_svn;     ///< The CPUSVN used to generate the PCK Signature used to certify the attestation key.
//     sgx_isv_svn_t cert_pce_isv_svn; ///< The PCE ISVSVN used to generate the PCK Signature used to certify the attestation key.
//     uint32_t cert_data_size;        ///< The size of the buffer pointed to by p_cert_data
//     uint8_t *p_cert_data;           ///< The certification data used for the quote.
//                                     ///todo: It is the assumed to be the PCK Cert Chain.  May want to change to support other cert types.
// } sgx_ql_config_t;

#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SgxQlConfig {
    pub version: SgxQlConfigVersion,
    pub cert_cpu_svn: SgxCpuSvn,
    pub cert_pce_isv_svn: SgxIsvSvn,
    pub cert_data_size: u32,
    pub cert_data: *mut u8,
}

impl SgxQlConfig {
    pub fn new(cpu_svn: [u8; 16], isv_svn: u16, cert: Vec<u8>) -> Self {
        // HOMEWORK: is there anything wrong here?
        // can we do it better here?
        // allocate memory for cert_data
        let cert_data = unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(cert.len(), 1)) };
        // copy cert to cert_data
        unsafe { std::ptr::copy_nonoverlapping(cert.as_ptr(), cert_data, cert.len()) };

        Self {
            version: SgxQlConfigVersion::SgxQlConfigVersion1,
            cert_cpu_svn: SgxCpuSvn {
                cpu_svn: cpu_svn,
            },
            cert_pce_isv_svn: SgxIsvSvn { isv_svn: isv_svn },
            cert_data_size: cert.len() as u32,
            cert_data: cert_data,
        }
    }
}

impl Drop for SgxQlConfig {
    fn drop(&mut self) {
        println!("free-ing cert_data");
        // free cert_data
        unsafe { std::alloc::dealloc(self.cert_data, std::alloc::Layout::from_size_align_unchecked(self.cert_data_size as usize, 1)) };
        self.cert_data_size = 0;
        self.cert_data = std::ptr::null_mut();
    }
}