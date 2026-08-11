# Security Policy

TenaxFS currently focuses on integrity against accidental corruption, incomplete
writes, interrupted erase, and modeled power loss.

CRC and checksums are not authentication. They do not protect against an
adversary who can intentionally modify flash contents.

Security-sensitive monotonic counters should use hardware support such as OTP,
eFuse, secure elements, or protected monotonic counters when the threat model
requires adversarial rollback protection. TenaxFS operational counters are not a
replacement for those mechanisms.

Please report security-sensitive issues privately to the maintainers once a
public contact exists. Until then, avoid publishing exploit details for active
vulnerabilities in unreleased code.

