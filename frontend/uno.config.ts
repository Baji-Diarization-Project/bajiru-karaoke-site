import { mergeConfigs, presetIcons, presetWind4 } from 'unocss'
import config from './.nuxt/uno.config.mjs'

export default mergeConfigs([
  config,
  {
    presets: [presetIcons(), presetWind4()],
  },
])
