# Unknown lower pair does not erase fixed-slot code identity

C156 SelectPrgPair incrementsguard,storesregister,JSR C160five-write serialroutine,decrementsguard,RTS. C160 shiftsA andSTAFFFF five times thenRTS. Both have standardbalancedreturnstack and noinlinearguments. Serialwrites determine an effective lower16KiBpair under alreadyestablishedMMC1 fixed-upper mode; fixed30/31 codeidentity staysknown. A's globalvalue mayremainunknown.

At a callerPC≥C000, normalreturn toPC+3 therefore supports context[null,null,30,31] rather than denyinganycontinuation. Follow fixed-slot instructions with exactphysicalidentity, but neverfetchbank0/26/etc through loweraliases unless a separatecontextcontract establishesit. No claim lowerpairunchangedorzero; existing explicitEA02/EA32/C931etc contracts remain stronger. Calls inlowerbanks do not getthis overridebecause their returnPCidentitywouldbeunknown.

Primary current source SelectPrgPair/WriteMmc1PrgSerial and knownmapper protocol, inheritedconfirmedfixed-upperexecutionviews. This is a conditionalreturncontract, notobservedexecutionorhardwarecompatibilityclaim.
