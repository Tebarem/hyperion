use crate::simulation::EntityKind;

impl EntityKind {
    pub const ALLAY: Self = Self(0i32);
    pub const AREA_EFFECT_CLOUD: Self = Self(1i32);
    pub const ARMOR_STAND: Self = Self(2i32);
    pub const ARROW: Self = Self(3i32);
    pub const AXOLOTL: Self = Self(4i32);
    pub const BAT: Self = Self(5i32);
    pub const BEE: Self = Self(6i32);
    pub const BLAZE: Self = Self(7i32);
    pub const BLOCK_DISPLAY: Self = Self(8i32);
    pub const BOAT: Self = Self(9i32);
    pub const CAMEL: Self = Self(10i32);
    pub const CAT: Self = Self(11i32);
    pub const CAVE_SPIDER: Self = Self(12i32);
    pub const CHEST_BOAT: Self = Self(13i32);
    pub const CHEST_MINECART: Self = Self(14i32);
    pub const CHICKEN: Self = Self(15i32);
    pub const COD: Self = Self(16i32);
    pub const COMMAND_BLOCK_MINECART: Self = Self(17i32);
    pub const COW: Self = Self(18i32);
    pub const CREEPER: Self = Self(19i32);
    pub const DOLPHIN: Self = Self(20i32);
    pub const DONKEY: Self = Self(21i32);
    pub const DRAGON_FIREBALL: Self = Self(22i32);
    pub const DROWNED: Self = Self(23i32);
    pub const EGG: Self = Self(24i32);
    pub const ELDER_GUARDIAN: Self = Self(25i32);
    pub const ENDERMAN: Self = Self(29i32);
    pub const ENDERMITE: Self = Self(30i32);
    pub const ENDER_DRAGON: Self = Self(27i32);
    pub const ENDER_PEARL: Self = Self(28i32);
    pub const END_CRYSTAL: Self = Self(26i32);
    pub const EVOKER: Self = Self(31i32);
    pub const EVOKER_FANGS: Self = Self(32i32);
    pub const EXPERIENCE_BOTTLE: Self = Self(33i32);
    pub const EXPERIENCE_ORB: Self = Self(34i32);
    pub const EYE_OF_ENDER: Self = Self(35i32);
    pub const FALLING_BLOCK: Self = Self(36i32);
    pub const FIREBALL: Self = Self(57i32);
    pub const FIREWORK_ROCKET: Self = Self(37i32);
    pub const FISHING_BOBBER: Self = Self(123i32);
    pub const FOX: Self = Self(38i32);
    pub const FROG: Self = Self(39i32);
    pub const FURNACE_MINECART: Self = Self(40i32);
    pub const GHAST: Self = Self(41i32);
    pub const GIANT: Self = Self(42i32);
    pub const GLOW_ITEM_FRAME: Self = Self(43i32);
    pub const GLOW_SQUID: Self = Self(44i32);
    pub const GOAT: Self = Self(45i32);
    pub const GUARDIAN: Self = Self(46i32);
    pub const HOGLIN: Self = Self(47i32);
    pub const HOPPER_MINECART: Self = Self(48i32);
    pub const HORSE: Self = Self(49i32);
    pub const HUSK: Self = Self(50i32);
    pub const ILLUSIONER: Self = Self(51i32);
    pub const INTERACTION: Self = Self(52i32);
    pub const IRON_GOLEM: Self = Self(53i32);
    pub const ITEM: Self = Self(54i32);
    pub const ITEM_DISPLAY: Self = Self(55i32);
    pub const ITEM_FRAME: Self = Self(56i32);
    pub const LEASH_KNOT: Self = Self(58i32);
    pub const LIGHTNING: Self = Self(59i32);
    pub const LLAMA: Self = Self(60i32);
    pub const LLAMA_SPIT: Self = Self(61i32);
    pub const MAGMA_CUBE: Self = Self(62i32);
    pub const MARKER: Self = Self(63i32);
    pub const MINECART: Self = Self(64i32);
    pub const MOOSHROOM: Self = Self(65i32);
    pub const MULE: Self = Self(66i32);
    pub const OCELOT: Self = Self(67i32);
    pub const PAINTING: Self = Self(68i32);
    pub const PANDA: Self = Self(69i32);
    pub const PARROT: Self = Self(70i32);
    pub const PHANTOM: Self = Self(71i32);
    pub const PIG: Self = Self(72i32);
    pub const PIGLIN: Self = Self(73i32);
    pub const PIGLIN_BRUTE: Self = Self(74i32);
    pub const PILLAGER: Self = Self(75i32);
    pub const PLAYER: Self = Self(122i32);
    pub const POLAR_BEAR: Self = Self(76i32);
    pub const POTION: Self = Self(77i32);
    pub const PUFFERFISH: Self = Self(78i32);
    pub const RABBIT: Self = Self(79i32);
    pub const RAVAGER: Self = Self(80i32);
    pub const SALMON: Self = Self(81i32);
    pub const SHEEP: Self = Self(82i32);
    pub const SHULKER: Self = Self(83i32);
    pub const SHULKER_BULLET: Self = Self(84i32);
    pub const SILVERFISH: Self = Self(85i32);
    pub const SKELETON: Self = Self(86i32);
    pub const SKELETON_HORSE: Self = Self(87i32);
    pub const SLIME: Self = Self(88i32);
    pub const SMALL_FIREBALL: Self = Self(89i32);
    pub const SNIFFER: Self = Self(90i32);
    pub const SNOWBALL: Self = Self(92i32);
    pub const SNOW_GOLEM: Self = Self(91i32);
    pub const SPAWNER_MINECART: Self = Self(93i32);
    pub const SPECTRAL_ARROW: Self = Self(94i32);
    pub const SPIDER: Self = Self(95i32);
    pub const SQUID: Self = Self(96i32);
    pub const STRAY: Self = Self(97i32);
    pub const STRIDER: Self = Self(98i32);
    pub const TADPOLE: Self = Self(99i32);
    pub const TEXT_DISPLAY: Self = Self(100i32);
    pub const TNT: Self = Self(101i32);
    pub const TNT_MINECART: Self = Self(102i32);
    pub const TRADER_LLAMA: Self = Self(103i32);
    pub const TRIDENT: Self = Self(104i32);
    pub const TROPICAL_FISH: Self = Self(105i32);
    pub const TURTLE: Self = Self(106i32);
    pub const VEX: Self = Self(107i32);
    pub const VILLAGER: Self = Self(108i32);
    pub const VINDICATOR: Self = Self(109i32);
    pub const WANDERING_TRADER: Self = Self(110i32);
    pub const WARDEN: Self = Self(111i32);
    pub const WITCH: Self = Self(112i32);
    pub const WITHER: Self = Self(113i32);
    pub const WITHER_SKELETON: Self = Self(114i32);
    pub const WITHER_SKULL: Self = Self(115i32);
    pub const WOLF: Self = Self(116i32);
    pub const ZOGLIN: Self = Self(117i32);
    pub const ZOMBIE: Self = Self(118i32);
    pub const ZOMBIE_HORSE: Self = Self(119i32);
    pub const ZOMBIE_VILLAGER: Self = Self(120i32);
    pub const ZOMBIFIED_PIGLIN: Self = Self(121i32);
}
