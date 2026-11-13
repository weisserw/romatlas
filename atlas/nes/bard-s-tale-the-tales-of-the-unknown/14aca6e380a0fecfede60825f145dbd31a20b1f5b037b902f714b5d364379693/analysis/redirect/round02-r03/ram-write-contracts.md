# Local RAM destination contracts

No generic indirectstore helper is globally assumed mapper-preserving. Conditional summaries E037/E0D4 are excluded from generic walker/wrapper summaries and accepted only inside reviewed D58E chain that first calls E03A.

92B5 parser initializes WorkPointer23/24=041E, uses only(23),Y writes, byteY bounds to051D. NestedE8AD,C2EF,9635 leave23/24 unchanged. E90A independently initializes same041E pointer;E973/C2EF leave it unchanged.

E03A sets FillPointer38/39=0500,Y6C before EC8A, bounds0500..056B. E1F9 sets CopyShortMemoryBlock destination0500 with count36, then FillPointer0536,Y36 and later copydestination041E,count1B. These locals validate those calls only.

E03A initializes scrollingcursor2E/2F=051B. E0D4/E10C writes currentcursor and cursorminus1B through32, with27column chunk and row transitions choosing0551; E1F9 scroll resets initialized RAM panel. Destinations remain inside RAM0500..056B under the normal panel contract. E037/E0D4 conditional contracts do not assert arbitrary corrupted RAM safety. D58E initializes viaE03A before callingE037, proving its composite bank-preserving return. Body/dependencies retained in lowerbank-summaries.json.

C4E7 fixedselector4 descriptor8016 enters94A3. All indirectwrites use WorkPointer23/24 initialized72D7,7462,771F. Independent primary generator vs initialized RAM output comparison establishes exact395,701,1417byte outputs, total2513, endpoints7462,771F,7CA8. All destinations below8000; nested9579 preserves local stack and increments same outputpointer. No mapperwrite. Primary evidence: analysis/redirect/round01-r02/template-comparison.json, templates_check.py and current sourceGenerateCartridgeRamHandlers/9579. Wrapper restores selector throughC4F4. This is a static output/destination proof; snapshot comparison is corroboration, not naturalreachability evidence.
