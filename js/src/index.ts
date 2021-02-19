import {hms as _hms, Runic as _Runic} from '../wasm/runic.js'

export type Hms = [number, number, number]

export const hms = (seconds: number): Hms => _hms(seconds) as Hms

export class OpenRunic {
  constructor(
    public readonly total: number
  ) {
  }

  hms(): Hms {
    return hms(this.total)
  }
}

export class Runic {
  private runic: _Runic

  constructor(
    public script: string
  ) {
    this.runic = _Runic.new(script)
  }

  timestamp(timestamp: number): Runic {
    this.runic.timestamp(timestamp)
    return this
  }

  offset(offset: number): Runic {
    this.runic.offset(offset)
    return this
  }

  describe(): OpenRunic {
    const openRunic = this.runic.describe()
    return new OpenRunic(openRunic.total())
  }
}
