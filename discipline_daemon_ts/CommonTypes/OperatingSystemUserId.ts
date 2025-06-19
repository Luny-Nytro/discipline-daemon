import { Unique } from "@Pkg/Unique";
import { Err, Ok, Tried } from "@Pkg/Tried";
import { GenericError } from "@Pkg/GenericError";
import { Integer } from "@Pkg/Integer";

export type OperatingSystemUserId = Unique<"App.Common.OperatingSystemUserId", number>

export const OperatingSystemUserId = {
  MIN_VALUE: 0,
  MAX_VALUE: 4294967295,

  new(rawUserId: number): Tried<OperatingSystemUserId, GenericError> {
    if (Integer.isIntegerAndInRange(
      rawUserId, 
      OperatingSystemUserId.MIN_VALUE,
      OperatingSystemUserId.MAX_VALUE
    )) {
      return Ok(Unique(rawUserId))
    }

    const error = GenericError.new("Create OperatingSystemUserId")
    GenericError.addMessage(error, "Argument 'rawUserId' is outside valid range")
    GenericError.addNamedAttachment(error, "Minimum valid value", OperatingSystemUserId.MIN_VALUE.toString())
    GenericError.addNamedAttachment(error, "Maximum valid value", OperatingSystemUserId.MAX_VALUE.toString())
    GenericError.addNamedAttachment(error, "Argument 'rawUserId'", (rawUserId as number).toString())
    return Err(error)
  },
  
  /**
   * 0	👑 Root user (Superuser of the galaxy!)
   */
  isRootUser(me: OperatingSystemUserId): boolean {
    return me === 0
  },

  /**
   * 1–99	🧙 System-reserved UIDs (static system users)
   */
  isStaticSystemUser(me: OperatingSystemUserId): boolean {
    return me >= 1 && me <= 99
  },

  /**
   * 100–999	🛠️ Dynamically assigned system/service users
   */
  isDynamicSystemUser(me: OperatingSystemUserId): boolean {
    return me >= 100 && me <= 999
  },

  /**
   * 1000–60000+	🧑‍🚀 Regular human/cosmic users (YOU!)
   */
  isRegularUser(me: OperatingSystemUserId): boolean {
    return me >= 1000 && me <= 60000
  },

  /**
   * 65534	👻 The mysterious nobody user (shadowy ghost!)
   */
  isNobody(me: OperatingSystemUserId): boolean {
    return me === 65534
  },

  /**
   * 65535	⚠️ Sometimes used as "invalid" (in 16-bit systems)
   */
  isInvalid(me: OperatingSystemUserId): boolean {
    return me === 65535
  },
}