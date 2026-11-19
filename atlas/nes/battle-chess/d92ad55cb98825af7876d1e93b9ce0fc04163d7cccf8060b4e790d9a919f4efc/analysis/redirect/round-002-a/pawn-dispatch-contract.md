# Pawn callback static contract

Precondition: initialized engine overlay, sideE6∈{00,80}, callback pointer high0D=82 from8CFB. The pawn initializer81E5 sets directioncursor fromside, then increments before lookup. Stepdelta consumer8B39,Y reads exactly indexedruns forY1..5 or81..85; fifthdelta00 terminates before callback. ForY1..4/81..84 callbacklow from8B34,Y is31/31/2B/1A in both runs. Thus8217 JMP($000C) has targets8231,822B,821A for this independently supported precondition. This proves static callbackstructure, not actual selectedside/path. No all256selector assumption. Primary decode26-8136.json plus current source EngineInitializePawnSteps, immutable byte rows8B34..8BBE and EngineClearOverlay8CFB.

Do not apply to uninitializedengine RAM. Proposed context retains26/27/30/31. Endpoints821A..8254 can follow concrete ordinary control paths; other indirectcontrols remain barriers.
