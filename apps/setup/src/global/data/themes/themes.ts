import { ThemeStyle, styles } from "./styles";
import { ThemeName } from "./enums";

export type ThemeItem = {
  name: ThemeName;
  label: string;
  styles: ThemeStyle;
};

export const themes: Record<ThemeName, ThemeItem> = {
  [ThemeName.BoldTech]: {
    name: ThemeName.BoldTech,
    label: "Bold Tech",
    styles: styles[ThemeName.BoldTech],
  },
  [ThemeName.Bubblegum]: {
    name: ThemeName.Bubblegum,
    label: "Bubblegum",
    styles: styles[ThemeName.Bubblegum],
  },
  [ThemeName.Caffeine]: {
    name: ThemeName.Caffeine,
    label: "Caffeine",
    styles: styles[ThemeName.Caffeine],
  },
  [ThemeName.Candyland]: {
    name: ThemeName.Candyland,
    label: "Candyland",
    styles: styles[ThemeName.Candyland],
  },
  [ThemeName.Catppuccin]: {
    name: ThemeName.Catppuccin,
    label: "Catppuccin",
    styles: styles[ThemeName.Catppuccin],
  },
  [ThemeName.Claude]: {
    name: ThemeName.Claude,
    label: "Claude",
    styles: styles[ThemeName.Claude],
  },
  [ThemeName.Claymorphism]: {
    name: ThemeName.Claymorphism,
    label: "Claymorphism",
    styles: styles[ThemeName.Claymorphism],
  },
  [ThemeName.CleanSlate]: {
    name: ThemeName.CleanSlate,
    label: "Clean Slate",
    styles: styles[ThemeName.CleanSlate],
  },
  [ThemeName.CosmicNight]: {
    name: ThemeName.CosmicNight,
    label: "Cosmic Night",
    styles: styles[ThemeName.CosmicNight],
  },
  [ThemeName.Cyberpunk]: {
    name: ThemeName.Cyberpunk,
    label: "Cyberpunk",
    styles: styles[ThemeName.Cyberpunk],
  },
  [ThemeName.ElegantLuxury]: {
    name: ThemeName.ElegantLuxury,
    label: "Elegant Luxury",
    styles: styles[ThemeName.ElegantLuxury],
  },
  [ThemeName.Graphite]: {
    name: ThemeName.Graphite,
    label: "Graphite",
    styles: styles[ThemeName.Graphite],
  },
  [ThemeName.KodamaGrove]: {
    name: ThemeName.KodamaGrove,
    label: "Kodama Grove",
    styles: styles[ThemeName.KodamaGrove],
  },
  [ThemeName.MidnightBloom]: {
    name: ThemeName.MidnightBloom,
    label: "Midnight Bloom",
    styles: styles[ThemeName.MidnightBloom],
  },
  [ThemeName.MochaMousse]: {
    name: ThemeName.MochaMousse,
    label: "Mocha Mousse",
    styles: styles[ThemeName.MochaMousse],
  },
  [ThemeName.ModernMinimal]: {
    name: ThemeName.ModernMinimal,
    label: "Modern Minimal",
    styles: styles[ThemeName.ModernMinimal],
  },
  [ThemeName.Mono]: {
    name: ThemeName.Mono,
    label: "Mono",
    styles: styles[ThemeName.Mono],
  },
  [ThemeName.Nature]: {
    name: ThemeName.Nature,
    label: "Nature",
    styles: styles[ThemeName.Nature],
  },
  [ThemeName.NeoBrutalism]: {
    name: ThemeName.NeoBrutalism,
    label: "Neo Brutalism",
    styles: styles[ThemeName.NeoBrutalism],
  },
  [ThemeName.NorthernLights]: {
    name: ThemeName.NorthernLights,
    label: "Northern Lights",
    styles: styles[ThemeName.NorthernLights],
  },
  [ThemeName.OceanBreeze]: {
    name: ThemeName.OceanBreeze,
    label: "Ocean Breeze",
    styles: styles[ThemeName.OceanBreeze],
  },
  [ThemeName.PastelDreams]: {
    name: ThemeName.PastelDreams,
    label: "Pastel Dreams",
    styles: styles[ThemeName.PastelDreams],
  },
  [ThemeName.Perpetuity]: {
    name: ThemeName.Perpetuity,
    label: "Perpetuity",
    styles: styles[ThemeName.Perpetuity],
  },
  [ThemeName.QuantumRose]: {
    name: ThemeName.QuantumRose,
    label: "Quantum Rose",
    styles: styles[ThemeName.QuantumRose],
  },
  [ThemeName.RetroArcade]: {
    name: ThemeName.RetroArcade,
    label: "Retro Arcade",
    styles: styles[ThemeName.RetroArcade],
  },
  [ThemeName.SolarDusk]: {
    name: ThemeName.SolarDusk,
    label: "Solar Dusk",
    styles: styles[ThemeName.SolarDusk],
  },
  [ThemeName.StarryNight]: {
    name: ThemeName.StarryNight,
    label: "Starry Night",
    styles: styles[ThemeName.StarryNight],
  },
  [ThemeName.SunsetHorizon]: {
    name: ThemeName.SunsetHorizon,
    label: "Sunset Horizon",
    styles: styles[ThemeName.SunsetHorizon],
  },
  [ThemeName.T3Chat]: {
    name: ThemeName.T3Chat,
    label: "T3 Chat",
    styles: styles[ThemeName.T3Chat],
  },
  [ThemeName.Tangerine]: {
    name: ThemeName.Tangerine,
    label: "Tangerine",
    styles: styles[ThemeName.Tangerine],
  },
  [ThemeName.Twitter]: {
    name: ThemeName.Twitter,
    label: "Twitter",
    styles: styles[ThemeName.Twitter],
  },
  [ThemeName.Vercel]: {
    name: ThemeName.Vercel,
    label: "Vercel",
    styles: styles[ThemeName.Vercel],
  },
  [ThemeName.VintagePaper]: {
    name: ThemeName.VintagePaper,
    label: "Vintage Paper",
    styles: styles[ThemeName.VintagePaper],
  },
} as const;
