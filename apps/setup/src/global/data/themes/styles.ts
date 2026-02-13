import { styles as boldTech } from "./styles/bold-tech.ts";
import { styles as bubblegum } from "./styles/bubblegum.ts";
import { styles as caffeine } from "./styles/caffeine.ts";
import { styles as candyland } from "./styles/candyland.ts";
import { styles as catppuccin } from "./styles/catppuccin.ts";
import { styles as claude } from "./styles/claude.ts";
import { styles as claymorphism } from "./styles/claymorphism.ts";
import { styles as cleanSlate } from "./styles/clean-slate.ts";
import { styles as cosmicNight } from "./styles/cosmic-night.ts";
import { styles as cyberpunk } from "./styles/cyberpunk.ts";
import { styles as elegantLuxury } from "./styles/elegant-luxury.ts";
import { styles as graphite } from "./styles/graphite.ts";
import { styles as kodamaGrove } from "./styles/kodama-grove.ts";
import { styles as midnightBloom } from "./styles/midnight-bloom.ts";
import { styles as mochaMousse } from "./styles/mocha-mousse.ts";
import { styles as modernMinimal } from "./styles/modern-minimal.ts";
import { styles as mono } from "./styles/mono.ts";
import { styles as nature } from "./styles/nature.ts";
import { styles as neoBrutalism } from "./styles/neo-brutalism.ts";
import { styles as northernLights } from "./styles/northern-lights.ts";
import { styles as oceanBreeze } from "./styles/ocean-breeze.ts";
import { styles as pastelDreams } from "./styles/pastel-dreams.ts";
import { styles as perpetuity } from "./styles/perpetuity.ts";
import { styles as quantumRose } from "./styles/quantum-rose.ts";
import { styles as retroArcade } from "./styles/retro-arcade.ts";
import { styles as solarDusk } from "./styles/solar-dusk.ts";
import { styles as starryNight } from "./styles/starry-night.ts";
import { styles as sunsetHorizon } from "./styles/sunset-horizon.ts";
import { styles as t3Chat } from "./styles/t3-chat.ts";
import { styles as tangerine } from "./styles/tangerine.ts";
import { styles as twitter } from "./styles/twitter.ts";
import { styles as vercel } from "./styles/vercel.ts";
import { styles as vintagePaper } from "./styles/vintage-paper.ts";

import { ThemeName } from "./enums";

export type ThemeStyle = { light: string, dark: string };

export const styles: Record<ThemeName, ThemeStyle> = {
  [ThemeName.BoldTech]: boldTech,
  [ThemeName.Bubblegum]: bubblegum,
  [ThemeName.Caffeine]: caffeine,
  [ThemeName.Candyland]: candyland,
  [ThemeName.Catppuccin]: catppuccin,
  [ThemeName.Claude]: claude,
  [ThemeName.Claymorphism]: claymorphism,
  [ThemeName.CleanSlate]: cleanSlate,
  [ThemeName.CosmicNight]: cosmicNight,
  [ThemeName.Cyberpunk]: cyberpunk,
  [ThemeName.ElegantLuxury]: elegantLuxury,
  [ThemeName.Graphite]: graphite,
  [ThemeName.KodamaGrove]: kodamaGrove,
  [ThemeName.MidnightBloom]: midnightBloom,
  [ThemeName.MochaMousse]: mochaMousse,
  [ThemeName.ModernMinimal]: modernMinimal,
  [ThemeName.Mono]: mono,
  [ThemeName.Nature]: nature,
  [ThemeName.NeoBrutalism]: neoBrutalism,
  [ThemeName.NorthernLights]: northernLights,
  [ThemeName.OceanBreeze]: oceanBreeze,
  [ThemeName.PastelDreams]: pastelDreams,
  [ThemeName.Perpetuity]: perpetuity,
  [ThemeName.QuantumRose]: quantumRose,
  [ThemeName.RetroArcade]: retroArcade,
  [ThemeName.SolarDusk]: solarDusk,
  [ThemeName.StarryNight]: starryNight,
  [ThemeName.SunsetHorizon]: sunsetHorizon,
  [ThemeName.T3Chat]: t3Chat,
  [ThemeName.Tangerine]: tangerine,
  [ThemeName.Twitter]: twitter,
  [ThemeName.Vercel]: vercel,
  [ThemeName.VintagePaper]: vintagePaper,
};
