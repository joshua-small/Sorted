<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  type ThemePref = "system" | "light" | "dark";

  type PuzzleSettings = {
    nColors: number;
    nEmptyContainers: number;
    containerVolume: number;
  };

  type BackendPuzzleSettings = {
    nColors: number;
    nEmpty: number;
    nVolume: number;
  };

  type PuzzleState = {
    containers: number[][];
    containerVolume: number;
  };

  type BackendPuzzleState = {
    vials: number[][];
    nVolume: number;
  };

  type Move = {
    src: number;
    dst: number;
  };

  type StateCheck = {
    status: "solved" | "stuck" | "inProgress";
    movesAvailable: number;
  };

  type PastPuzzle = {
    id: string;
    finishedAt: string;
    puzzleSeed: string;
    settings: PuzzleSettings;
    singleMode: boolean;
    initialState: PuzzleState;
    userMoves: number;
    totalMoves: number;
    elapsedSeconds: number;
    endedAs: "solved" | "stuck";
  };

  type BoardLayoutOption = {
    rows: number;
    counts: number[];
  };

  type KeyLayoutOption = {
    rows: number;
    keys: string[];
  };

  type RuntimeWindow = Window & {
    __TAURI_INTERNALS__?: unknown;
  };

  type UserOptionsConfig = {
    theme?: ThemePref;
    nColors?: number;
    nEmptyContainers?: number;
    containerVolume?: number;
    singleBlockMode?: boolean;
    autoNextEnabled?: boolean;
    autoNextSeconds?: number;
  };

  const FALLBACK_KEYS = "1234567890QWERTYUIO";
  const HISTORY_KEY = "sorted_history_v1";
  const USER_OPTIONS_CONFIG_PATH = "/user-options.config.json";
  const BOARD_LAYOUTS: Record<number, BoardLayoutOption[]> = {
    4: [{ rows: 1, counts: [4] }],
    5: [{ rows: 1, counts: [5] }],
    6: [{ rows: 1, counts: [6] }],
    7: [{ rows: 1, counts: [7] }, { rows: 2, counts: [4, 3] }],
    8: [{ rows: 1, counts: [8] }, { rows: 2, counts: [4, 4] }],
    9: [{ rows: 1, counts: [9] }, { rows: 2, counts: [5, 4] }, { rows: 3, counts: [3, 3, 3] }],
    10: [{ rows: 1, counts: [10] }, { rows: 2, counts: [5, 5] }, { rows: 3, counts: [4, 3, 3] }],
    11: [{ rows: 2, counts: [6, 5] }, { rows: 3, counts: [4, 4, 3] }],
    12: [{ rows: 2, counts: [6, 6] }, { rows: 3, counts: [4, 4, 4] }],
    13: [{ rows: 2, counts: [7, 6] }, { rows: 3, counts: [5, 4, 4] }],
    14: [{ rows: 2, counts: [7, 7] }, { rows: 3, counts: [5, 5, 4] }],
    15: [{ rows: 2, counts: [8, 7] }, { rows: 3, counts: [5, 5, 5] }],
    16: [{ rows: 2, counts: [8, 8] }, { rows: 3, counts: [6, 5, 5] }],
    17: [{ rows: 2, counts: [9, 8] }, { rows: 3, counts: [6, 6, 5] }]
  };
  const BOARD_ITEM_TARGET_WIDTH = 104;
  const BOARD_GAP_PX = 16;

  const KEY_LAYOUT_BY_NCONTAINERS: Record<number, KeyLayoutOption[]> = {
    3: [{ rows: 1, keys: ["1", "2", "3"] }],
    4: [{ rows: 1, keys: ["1", "2", "3", "4"] }],
    5: [{ rows: 1, keys: ["1", "2", "3", "4", "5"] }],
    6: [{ rows: 1, keys: ["1", "2", "3", "4", "5", "6"] }],
    7: [{ rows: 1, keys: ["1", "2", "3", "4", "5", "6", "7"] }, { rows: 2, keys: ["1", "2", "3", "4", "Q", "W", "E"] }],
    8: [{ rows: 1, keys: ["1", "2", "3", "4", "5", "6", "7", "8"] }, { rows: 2, keys: ["1", "2", "3", "4", "Q", "W", "E", "R"] }],
    9: [{ rows: 1, keys: ["1", "2", "3", "4", "5", "6", "7", "8", "9"] }, { rows: 2, keys: ["1", "2", "3", "4", "5", "Q", "W", "E", "R"] }, { rows: 3, keys: ["1", "2", "3", "Q", "W", "E", "A", "S", "D"] }],
    10: [{ rows: 2, keys: ["1", "2", "3", "4", "5", "Q", "W", "E", "R", "T"] }, { rows: 1, keys: ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"] }, { rows: 3, keys: ["1", "2", "3", "4", "Q", "W", "E", "A", "S", "D"] }],
    11: [{ rows: 2, keys: ["1", "2", "3", "4", "5", "6", "Q", "W", "E", "R", "T"] }, { rows: 3, keys: ["1", "2", "3", "4", "Q", "W", "E", "R", "A", "S", "D"] }],
    12: [{ rows: 2, keys: ["1", "2", "3", "4", "5", "6", "Q", "W", "E", "R", "T", "Y"] }, { rows: 3, keys: ["1", "2", "3", "4", "Q", "W", "E", "R", "A", "S", "D", "F"] }],
    13: [{ rows: 2, keys: ["1", "2", "3", "4", "5", "6", "7", "Q", "W", "E", "R", "T", "Y"] }, { rows: 3, keys: ["1", "2", "3", "4", "5", "Q", "W", "E", "R", "A", "S", "D", "F"] }],
    14: [{ rows: 2, keys: ["1", "2", "3", "4", "5", "6", "7", "Q", "W", "E", "R", "T", "Y", "U"] }, { rows: 3, keys: ["1", "2", "3", "4", "5", "Q", "W", "E", "R", "T", "A", "S", "D", "F"] }],
    15: [{ rows: 2, keys: ["1", "2", "3", "4", "5", "6", "7", "8", "Q", "W", "E", "R", "T", "Y", "U"] }, { rows: 3, keys: ["1", "2", "3", "4", "5", "Q", "W", "E", "R", "T", "A", "S", "D", "F", "G"] }],
    16: [{ rows: 2, keys: ["1", "2", "3", "4", "5", "6", "7", "8", "Q", "W", "E", "R", "T", "Y", "U", "I"] }, { rows: 3, keys: ["1", "2", "3", "4", "5", "6", "Q", "W", "E", "R", "T", "A", "S", "D", "F", "G"] }],
    17: [{ rows: 2, keys: ["1", "2", "3", "4", "5", "6", "7", "8", "Q", "W", "E", "R", "T", "Y", "U", "I", "O"] }, { rows: 3, keys: ["1", "2", "3", "4", "5", "6", "Q", "W", "E", "R", "T", "Y", "A", "S", "D", "F", "G"] }]
  };

  const palette = [
    "#ffffff",
    "#0000ff",
    "#ff0000",
    "#00ff00",
    "#ffff00",
    "#ff00ff",
    "#00ffff",
    "#808080",
    "#eb5e8c",
    "#808000",
    "#c16c2d",
    "#ffac7a",
    "#01621a",
    "#008cff",
    "#1a1a1a"
  ];

  const themeTokens = {
    light: {
      bg: "#f5f5f5",
      surface: "#ffffff",
      ink: "#101217",
      muted: "#5f6572",
      line: "#cfd4de",
      cellStroke: "rgba(0, 0, 0, 0.35)"
    },
    dark: {
      bg: "#1a1d22",
      surface: "#252a31",
      ink: "#f2f4f8",
      muted: "#b8becb",
      line: "#495263",
      cellStroke: "rgba(255, 255, 255, 0.35)"
    }
  } as const;

  let themePref = $state<ThemePref>("system");
  let systemDark = $state(false);

  let settings = $state<PuzzleSettings>({ nColors: 9, nEmptyContainers: 2, containerVolume: 5 });
  let singleMode = $state(false);
  let autoNextEnabled = $state(true);
  let autoNextSeconds = $state(12);
  let showHistory = $state(false);
  let showOptions = $state(false);

  let board = $state<PuzzleState>({ containers: [], containerVolume: 5 });
  let initialState = $state<PuzzleState | null>(null);
  let moveHistory = $state<Move[]>([]);
  let redoHistory = $state<Move[]>([]);
  let selectedSrc = $state<number | null>(null);
  let userMoves = $state(0);
  let totalMoves = $state(0);
  let elapsedSeconds = $state(0);
  let statusLine = $state("Loading puzzle...");
  let seedInput = $state("");
  let currentSeed = $state("");
  // TODO: Add optional advanced solver panel in menu and a next-move hint action.
  // TODO: Add future art/audio packs (containers, backgrounds, materials, effects, music)
  // while keeping the current clean rectangle style for MVP.
  // TODO: Re-implement end-state optimal solver asynchronously without UI freezes.

  let overlayOpen = $state(false);
  let overlayStatus = $state<"solved" | "stuck">("stuck");
  let countdown = $state(12);
  let countdownIntervalId: ReturnType<typeof setInterval> | null = null;
  let gameTimerIntervalId: ReturnType<typeof setInterval> | null = null;
  let boardViewportWidth = $state(0);

  let pastPuzzles = $state<PastPuzzle[]>([]);

  const effectiveDark = $derived(
    themePref === "system" ? systemDark : themePref === "dark"
  );

  const appliedTheme = $derived(effectiveDark ? "dark" : "light");

  const activeKeys = $derived.by(() => {
    const nContainers = board.containers.length || (settings.nColors + settings.nEmptyContainers);
    const rowCount = boardRowCounts.length > 0 ? boardRowCounts.length : 1;
    const options = KEY_LAYOUT_BY_NCONTAINERS[nContainers];
    const selected = options
      ? (options.find((entry) => entry.rows === rowCount) ?? options[0])
      : null;
    const base = selected
      ? [...selected.keys]
      : FALLBACK_KEYS.slice(0, nContainers).split("");
    const all = FALLBACK_KEYS.split("");
    for (const key of all) {
      if (base.length >= nContainers) {
        break;
      }
      if (!base.includes(key)) {
        base.push(key);
      }
    }
    return base.slice(0, nContainers);
  });

  const currentPuzzleSeed = $derived(currentSeed || "n/a");
  const totalContainers = $derived(settings.nColors + settings.nEmptyContainers);
  const boardRowCounts = $derived.by(() => pickBoardLayout(board.containers.length, boardViewportWidth));
  const boardRows = $derived.by(() => {
    const rows: Array<Array<{ container: number[]; index: number }>> = [];
    let startIndex = 0;
    for (const count of boardRowCounts) {
      const row: Array<{ container: number[]; index: number }> = [];
      for (let offset = 0; offset < count; offset += 1) {
        const index = startIndex + offset;
        const container = board.containers[index];
        if (container) {
          row.push({ container, index });
        }
      }
      if (row.length > 0) {
        rows.push(row);
      }
      startIndex += count;
    }
    return rows;
  });

  function toBackendSettings(ui: PuzzleSettings): BackendPuzzleSettings {
    return {
      nColors: ui.nColors,
      nEmpty: ui.nEmptyContainers,
      nVolume: ui.containerVolume
    };
  }

  function fromBackendState(state: BackendPuzzleState): PuzzleState {
    return {
      containers: state.vials.map((v) => [...v]),
      containerVolume: state.nVolume
    };
  }

  function toBackendState(state: PuzzleState): BackendPuzzleState {
    return {
      vials: state.containers.map((v) => [...v]),
      nVolume: state.containerVolume
    };
  }

  function cloneState(state: PuzzleState): PuzzleState {
    return {
      containerVolume: state.containerVolume,
      containers: state.containers.map((v) => [...v])
    };
  }

  function pickBoardLayout(nContainers: number, availableWidth: number): number[] {
    const options = BOARD_LAYOUTS[nContainers];
    if (!options || options.length === 0) {
      return [nContainers];
    }
    if (availableWidth <= 0) {
      return options[0].counts;
    }

    for (const option of options) {
      const widestRow = Math.max(...option.counts);
      const requiredWidth = widestRow * BOARD_ITEM_TARGET_WIDTH + (widestRow - 1) * BOARD_GAP_PX;
      if (requiredWidth <= availableWidth) {
        return option.counts;
      }
    }

    return options[options.length - 1].counts;
  }

  function keyLabel(i: number): string {
    return activeKeys[i] ?? "?";
  }

  function keyToIndex(key: string): number {
    return activeKeys.indexOf(key.toUpperCase());
  }

  function isTextEntryTarget(target: EventTarget | null): boolean {
    if (!(target instanceof HTMLElement)) {
      return false;
    }

    const tagName = target.tagName;
    if (target.isContentEditable) {
      return true;
    }

    return tagName === "INPUT" || tagName === "TEXTAREA" || tagName === "SELECT";
  }

  function randomSeedHex(): string {
    const values = new Uint32Array(2);
    crypto.getRandomValues(values);
    const hi = values[0].toString(16).padStart(8, "0");
    const lo = values[1].toString(16).padStart(8, "0");
    return `${hi}${lo}`.toUpperCase();
  }

  function normalizeSeed(input: string): string | null {
    const seed = input.trim().toUpperCase();
    if (!/^[0-9A-F]{16}$/.test(seed)) {
      return null;
    }
    return seed;
  }

  function topRun(container: number[]): { color: number; run: number } | null {
    if (container.length === 0) {
      return null;
    }
    const color = container[container.length - 1];
    let run = 0;
    for (let i = container.length - 1; i >= 0; i -= 1) {
      if (container[i] === color) {
        run += 1;
      } else {
        break;
      }
    }
    return { color, run };
  }

  function isTauriRuntime(): boolean {
    if (typeof window === "undefined") {
      return false;
    }
    return !!(window as RuntimeWindow).__TAURI_INTERNALS__;
  }

  function seedToU64(seedHex: string): bigint {
    const normalized = seedHex.trim().toUpperCase();
    if (!/^[0-9A-F]{16}$/.test(normalized)) {
      return 0x9e3779b97f4a7c15n;
    }
    return BigInt(`0x${normalized}`);
  }

  function makeSeededRng(seedHex: string): () => number {
    const mask = 0xffffffffffffffffn;
    const add = 0x9e3779b97f4a7c15n;
    let state = seedToU64(seedHex) & mask;

    return () => {
      state = (state + add) & mask;
      let z = state;
      z = ((z ^ (z >> 30n)) * 0xbf58476d1ce4e5b9n) & mask;
      z = ((z ^ (z >> 27n)) * 0x94d049bb133111ebn) & mask;
      z ^= z >> 31n;
      return Number(z >> 11n) / 9007199254740992;
    };
  }

  function shuffleInPlace(values: number[], rng: () => number): void {
    for (let i = values.length - 1; i > 0; i -= 1) {
      const j = Math.floor(rng() * (i + 1));
      [values[i], values[j]] = [values[j], values[i]];
    }
  }

  function isSolvedState(state: PuzzleState): boolean {
    for (const container of state.containers) {
      if (container.length === 0) {
        continue;
      }
      if (container.length !== state.containerVolume) {
        return false;
      }
      const first = container[0];
      for (const color of container) {
        if (color !== first) {
          return false;
        }
      }
    }
    return true;
  }

  function availableMoveCount(state: PuzzleState): number {
    let count = 0;
    for (let src = 0; src < state.containers.length; src += 1) {
      for (let dst = 0; dst < state.containers.length; dst += 1) {
        if (canApplyMove(state, src, dst)) {
          count += 1;
        }
      }
    }
    return count;
  }

  function fallbackCheckState(state: BackendPuzzleState): StateCheck {
    const uiState = fromBackendState(state);
    if (isSolvedState(uiState)) {
      return { status: "solved", movesAvailable: 0 };
    }

    const movesAvailable = availableMoveCount(uiState);
    if (movesAvailable === 0) {
      return { status: "stuck", movesAvailable };
    }
    return { status: "inProgress", movesAvailable };
  }

  function fallbackRandomPuzzle(settingsInput: BackendPuzzleSettings, seedHex: string): BackendPuzzleState {
    const rng = makeSeededRng(seedHex);
    const nColors = Math.max(2, Math.min(14, Math.trunc(settingsInput.nColors || 2)));
    const nEmpty = Math.max(1, Math.min(3, Math.trunc(settingsInput.nEmpty || 1)));
    const nVolume = Math.max(2, Math.min(20, Math.trunc(settingsInput.nVolume || 2)));

    const pool: number[] = [];
    for (let color = 1; color <= nColors; color += 1) {
      for (let i = 0; i < nVolume; i += 1) {
        pool.push(color);
      }
    }
    shuffleInPlace(pool, rng);

    const containers: number[][] = Array.from({ length: nColors + nEmpty }, () => []);
    let cursor = 0;
    for (let vial = 0; vial < nColors; vial += 1) {
      for (let i = 0; i < nVolume; i += 1) {
        containers[vial].push(pool[cursor]);
        cursor += 1;
      }
    }

    return {
      vials: containers,
      nVolume: nVolume
    };
  }

  async function randomPuzzleSeeded(settingsInput: BackendPuzzleSettings, seedHex: string): Promise<BackendPuzzleState> {
    // Use one deterministic generator in both desktop and browser so seeds match everywhere.
    return fallbackRandomPuzzle(settingsInput, seedHex);
  }

  async function checkState(state: BackendPuzzleState, singleModeInput: boolean): Promise<StateCheck> {
    if (isTauriRuntime()) {
      return invoke<StateCheck>("check_state", { state, singleMode: singleModeInput });
    }
    return fallbackCheckState(state);
  }

  function canApplyMove(state: PuzzleState, src: number, dst: number): boolean {
    if (src === dst) {
      return false;
    }
    const srcContainer = state.containers[src];
    const dstContainer = state.containers[dst];
    if (!srcContainer || !dstContainer || srcContainer.length === 0 || dstContainer.length >= state.containerVolume) {
      return false;
    }

    const sTop = topRun(srcContainer);
    if (!sTop) {
      return false;
    }

    if (dstContainer.length === 0) {
      const srcSingleColor = sTop.run === srcContainer.length;
      return !srcSingleColor;
    }

    const dTop = topRun(dstContainer);
    return !!dTop && dTop.color === sTop.color;
  }

  function applyMove(state: PuzzleState, move: Move): PuzzleState | null {
    if (!canApplyMove(state, move.src, move.dst)) {
      return null;
    }

    const next = cloneState(state);
    const srcContainer = next.containers[move.src];
    const dstContainer = next.containers[move.dst];
    const srcTop = topRun(srcContainer);
    if (!srcTop) {
      return null;
    }

    const amount = singleMode ? 1 : Math.min(srcTop.run, state.containerVolume - dstContainer.length);
    for (let i = 0; i < amount; i += 1) {
      const val = srcContainer.pop();
      if (val === undefined) {
        return null;
      }
      dstContainer.push(val);
    }
    return next;
  }

  function formatMove(move: Move): string {
    return `${move.src + 1}→${move.dst + 1}`;
  }

  function padNumber(value: number): string {
    return value.toString().padStart(2, "0");
  }

  function toFiniteNumber(value: unknown): number | null {
    if (typeof value !== "number" || !Number.isFinite(value)) {
      return null;
    }
    return value;
  }

  function clampInteger(value: number, min: number, max: number): number {
    return Math.max(min, Math.min(max, Math.trunc(value)));
  }

  function applyUserOptionsConfig(config: UserOptionsConfig): void {
    if (config.theme === "system" || config.theme === "light" || config.theme === "dark") {
      themePref = config.theme;
    }

    if (typeof config.singleBlockMode === "boolean") {
      singleMode = config.singleBlockMode;
    }

    if (typeof config.autoNextEnabled === "boolean") {
      autoNextEnabled = config.autoNextEnabled;
    }

    const configAutoNextSeconds = toFiniteNumber(config.autoNextSeconds);
    if (configAutoNextSeconds !== null) {
      autoNextSeconds = clampInteger(configAutoNextSeconds, 0, 60);
    }

    const configuredColors = toFiniteNumber(config.nColors);
    if (configuredColors !== null) {
      settings.nColors = clampInteger(configuredColors, 2, 14);
    }

    const configuredEmptyContainers = toFiniteNumber(config.nEmptyContainers);
    if (configuredEmptyContainers !== null) {
      settings.nEmptyContainers = clampInteger(configuredEmptyContainers, 1, 3);
    }

    const configuredVolume = toFiniteNumber(config.containerVolume);
    if (configuredVolume !== null) {
      settings.containerVolume = clampInteger(configuredVolume, 2, 20);
    }
  }

  async function loadUserOptionsConfig(): Promise<void> {
    try {
      const response = await fetch(USER_OPTIONS_CONFIG_PATH, { cache: "no-store" });
      if (!response.ok) {
        return;
      }

      const parsed = (await response.json()) as unknown;
      if (!parsed || typeof parsed !== "object") {
        return;
      }

      applyUserOptionsConfig(parsed as UserOptionsConfig);
    } catch {
      // Missing or malformed user config should never block startup.
    }
  }

  function formatDuration(totalSeconds: number): string {
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;
    if (hours > 0) {
      return `${padNumber(hours)}:${padNumber(minutes)}:${padNumber(seconds)}`;
    }
    return `${padNumber(minutes)}:${padNumber(seconds)}`;
  }

  function formatDateTime(timestamp: string): string {
    const value = new Date(timestamp);
    return `${value.getFullYear()}-${padNumber(value.getMonth() + 1)}-${padNumber(value.getDate())} ${padNumber(value.getHours())}:${padNumber(value.getMinutes())}:${padNumber(value.getSeconds())}`;
  }

  function startGameTimer(): void {
    clearGameTimer();
    gameTimerIntervalId = setInterval(() => {
      elapsedSeconds += 1;
    }, 1000);
  }

  function clearGameTimer(): void {
    if (gameTimerIntervalId) {
      clearInterval(gameTimerIntervalId);
      gameTimerIntervalId = null;
    }
  }

  function saveHistory(): void {
    localStorage.setItem(HISTORY_KEY, JSON.stringify(pastPuzzles));
  }

  function toggleOptionsOverlay(): void {
    showOptions = !showOptions;
    if (showOptions) {
      showHistory = false;
    }
  }

  function toggleHistoryOverlay(): void {
    showHistory = !showHistory;
    if (showHistory) {
      showOptions = false;
    }
  }

  function clearHistory(): void {
    if (!window.confirm("Clear all past puzzle history?")) {
      return;
    }
    pastPuzzles = [];
    saveHistory();
    statusLine = "Past puzzle history cleared.";
  }

  function loadHistory(): void {
    const raw = localStorage.getItem(HISTORY_KEY);
    if (!raw) {
      return;
    }
    try {
      const parsed = JSON.parse(raw) as Array<PastPuzzle & { puzzleHash?: string; settings?: any; totalMoves?: number; elapsedSeconds?: number }>;
      pastPuzzles = parsed.map((entry) => ({
        ...entry,
        puzzleSeed: entry.puzzleSeed ?? entry.puzzleHash ?? "",
        totalMoves: entry.totalMoves ?? entry.userMoves ?? 0,
        elapsedSeconds: entry.elapsedSeconds ?? 0,
        settings: {
          nColors: entry.settings?.nColors ?? 9,
          nEmptyContainers:
            entry.settings?.nEmptyContainers ??
            ((entry.settings?.nContainers ?? (entry.settings?.nColors ?? 9) + 2) - (entry.settings?.nColors ?? 9)),
          containerVolume: entry.settings?.containerVolume ?? entry.settings?.nVolume ?? 5
        }
      }));
    } catch {
      pastPuzzles = [];
    }
  }

  async function newRandomPuzzle(): Promise<void> {
    clearCountdown();
    clearGameTimer();
    overlayOpen = false;
    selectedSrc = null;
    moveHistory = [];
    redoHistory = [];
    userMoves = 0;
    totalMoves = 0;
    elapsedSeconds = 0;
    const seedHex = randomSeedHex();
    try {
      const seeded = await randomPuzzleSeeded(toBackendSettings(settings), seedHex);
      board = fromBackendState(seeded);
    } catch {
      statusLine = "Failed to generate puzzle.";
      return;
    }
    currentSeed = seedHex;
    seedInput = seedHex;
    initialState = cloneState(board);
    statusLine = "Puzzle loaded. Pick source then target container.";
  }

  async function applyUserMove(move: Move): Promise<void> {
    const updated = applyMove(board, move);
    if (!updated) {
      selectedSrc = move.src;
      statusLine = "Invalid move. Source kept selected.";
      return;
    }

    selectedSrc = null;
    if (!gameTimerIntervalId) {
      startGameTimer();
    }
    board = updated;
    moveHistory = [...moveHistory, move];
    redoHistory = [];
    userMoves += 1;
    totalMoves += 1;
    statusLine = `Move ${userMoves}: ${formatMove(move)}`;

    const check = await checkState(toBackendState(board), singleMode);
    if (check.status === "solved" || check.status === "stuck") {
      await openStats(check.status);
    }
  }

  async function onContainerClick(index: number): Promise<void> {
    if (overlayOpen) {
      return;
    }
    if (selectedSrc === null) {
      selectedSrc = index;
      statusLine = `Source container: ${index + 1}`;
      return;
    }

    if (selectedSrc === index) {
      selectedSrc = null;
      statusLine = "Selection cleared.";
      return;
    }

    await applyUserMove({ src: selectedSrc, dst: index });
  }

  function toggleThemeQuick(): void {
    if (themePref === "system") {
      themePref = effectiveDark ? "light" : "dark";
      return;
    }
    themePref = themePref === "dark" ? "light" : "dark";
  }

  async function onKeydown(ev: KeyboardEvent): Promise<void> {
    if (isTextEntryTarget(ev.target)) {
      return;
    }

    if ((ev.ctrlKey || ev.metaKey) && ev.key.toLowerCase() === "z") {
      ev.preventDefault();
      if (ev.shiftKey) {
        redoMove();
      } else {
        undoMove();
      }
      return;
    }

    if (overlayOpen) {
      if (overlayStatus === "solved" && (ev.key === " " || ev.key === "Spacebar" || ev.key === "Enter")) {
        ev.preventDefault();
        await newRandomPuzzle();
      }
      return;
    }

    if (ev.key === "Escape" && (showOptions || showHistory)) {
      ev.preventDefault();
      showOptions = false;
      showHistory = false;
      statusLine = "Overlay closed.";
      return;
    }

    if (ev.key === "ArrowLeft") {
      ev.preventDefault();
      undoMove();
      return;
    }
    if (ev.key === "ArrowRight") {
      ev.preventDefault();
      redoMove();
      return;
    }
    if (ev.key.toLowerCase() === "m") {
      ev.preventDefault();
      showOptions = true;
      return;
    }
    if (ev.key.toLowerCase() === "l") {
      ev.preventDefault();
      toggleThemeQuick();
      return;
    }
    if (ev.key === "Escape") {
      ev.preventDefault();
      if (selectedSrc !== null) {
        selectedSrc = null;
        statusLine = "Selection cleared.";
      } else {
        showOptions = true;
        statusLine = "Options opened.";
      }
      return;
    }
    if (ev.key === "Enter" || ev.key === "Delete" || ev.key === "Backspace") {
      ev.preventDefault();
      confirmRestartCurrentPuzzle();
      return;
    }

    const containerIndex = keyToIndex(ev.key);
    if (containerIndex < 0 || containerIndex >= board.containers.length) {
      return;
    }

    if (selectedSrc === null) {
      selectedSrc = containerIndex;
      statusLine = `Source container: ${containerIndex + 1}`;
      return;
    }

    if (selectedSrc === containerIndex) {
      selectedSrc = null;
      statusLine = "Selection cleared.";
      return;
    }

    await applyUserMove({ src: selectedSrc, dst: containerIndex });
  }

  function undoMove(): void {
    if (moveHistory.length === 0 || !initialState) {
      return;
    }

    const nextHistory = moveHistory.slice(0, -1);
    let rebuilt = cloneState(initialState);
    for (const move of nextHistory) {
      const applied = applyMove(rebuilt, move);
      if (!applied) {
        statusLine = "Undo failed due to inconsistent state.";
        return;
      }
      rebuilt = applied;
    }

    board = rebuilt;
    const undoneMove = moveHistory[moveHistory.length - 1];
    moveHistory = nextHistory;
    redoHistory = undoneMove ? [...redoHistory, undoneMove] : redoHistory;
    userMoves = nextHistory.length;
    totalMoves += 1;
    selectedSrc = null;
    statusLine = "Undid last move.";
  }

  function redoMove(): void {
    if (redoHistory.length === 0) {
      return;
    }

    const move = redoHistory[redoHistory.length - 1];
    const next = applyMove(board, move);
    if (!next) {
      statusLine = "Redo failed.";
      return;
    }

    board = next;
    moveHistory = [...moveHistory, move];
    redoHistory = redoHistory.slice(0, -1);
    userMoves = moveHistory.length;
    totalMoves += 1;
    selectedSrc = null;
    statusLine = `Redid move: ${formatMove(move)}`;
  }

  function restartCurrentPuzzle(): void {
    if (!initialState) {
      return;
    }

    clearCountdown();
    clearGameTimer();
    overlayOpen = false;
    board = cloneState(initialState);
    moveHistory = [];
    redoHistory = [];
    userMoves = 0;
    totalMoves = 0;
    elapsedSeconds = 0;
    selectedSrc = null;
    statusLine = "Current puzzle restarted.";
  }

  function confirmRestartCurrentPuzzle(): void {
    if (!initialState) {
      return;
    }

    if (window.confirm("Restart the current puzzle?")) {
      restartCurrentPuzzle();
    }
  }

  async function playSeedPuzzle(): Promise<void> {
    const seedHex = normalizeSeed(seedInput);
    if (!seedHex) {
      statusLine = "Invalid seed. Use exactly 16 hex chars.";
      return;
    }

    let seeded: BackendPuzzleState;
    try {
      seeded = await randomPuzzleSeeded(toBackendSettings(settings), seedHex);
    } catch {
      statusLine = "Failed to load puzzle from seed.";
      return;
    }

    clearCountdown();
    clearGameTimer();
    board = fromBackendState(seeded);
    initialState = cloneState(fromBackendState(seeded));
    currentSeed = seedHex;
    seedInput = seedHex;
    moveHistory = [];
    redoHistory = [];
    userMoves = 0;
    totalMoves = 0;
    elapsedSeconds = 0;
    selectedSrc = null;
    overlayOpen = false;
    statusLine = "Loaded puzzle from seed.";
  }

  async function copyCurrentSeed(): Promise<void> {
    if (!currentSeed) {
      return;
    }
    try {
      await navigator.clipboard.writeText(currentSeed);
      statusLine = "Copied current puzzle seed.";
    } catch {
      statusLine = "Failed to copy puzzle seed.";
    }
  }

  async function pasteSeedFromClipboard(): Promise<void> {
    try {
      const pasted = await navigator.clipboard.readText();
      seedInput = pasted.trim().toUpperCase();
      statusLine = "Pasted seed from clipboard.";
    } catch {
      statusLine = "Failed to paste puzzle seed.";
    }
  }

  function clearCountdown(): void {
    if (countdownIntervalId) {
      clearInterval(countdownIntervalId);
      countdownIntervalId = null;
    }
  }

  function startCountdown(): void {
    clearCountdown();
    if (!autoNextEnabled) {
      return;
    }
    countdown = autoNextSeconds;
    if (countdown <= 0) {
      void newRandomPuzzle();
      return;
    }
    countdownIntervalId = setInterval(async () => {
      countdown -= 1;
      if (countdown <= 0) {
        clearCountdown();
        await newRandomPuzzle();
      }
    }, 1000);
  }

  function addPastPuzzle(result: { endedAs: "solved" | "stuck" }): void {
    if (!initialState) {
      return;
    }
    // TODO: Re-introduce PB metrics with retroactive recalculation across all solved history entries.
    const entry: PastPuzzle = {
      id: crypto.randomUUID(),
      finishedAt: new Date().toISOString(),
      puzzleSeed: currentSeed,
      settings: { ...settings },
      singleMode,
      initialState: cloneState(initialState),
      userMoves,
      totalMoves,
      elapsedSeconds,
      endedAs: result.endedAs
    };
    pastPuzzles = [entry, ...pastPuzzles].slice(0, 100);
    saveHistory();
  }

  async function openStats(endedAs: "solved" | "stuck"): Promise<void> {
    clearGameTimer();
    overlayOpen = true;
    overlayStatus = endedAs;
    addPastPuzzle({ endedAs });
    if (endedAs === "solved") {
      startCountdown();
    } else {
      clearCountdown();
    }
  }

  function playAgain(entry: PastPuzzle): void {
    clearCountdown();
    clearGameTimer();
    settings = { ...entry.settings };
    singleMode = entry.singleMode;
    board = cloneState(entry.initialState);
    initialState = cloneState(entry.initialState);
    currentSeed = entry.puzzleSeed;
    seedInput = entry.puzzleSeed;
    moveHistory = [];
    redoHistory = [];
    userMoves = 0;
    totalMoves = 0;
    elapsedSeconds = 0;
    selectedSrc = null;
    overlayOpen = false;
    showHistory = false;
    statusLine = "Loaded previous puzzle. Good luck.";
  }

  $effect(() => {
    const clampedColors = Math.max(2, Math.min(14, Math.trunc(settings.nColors || 2)));
    if (settings.nColors !== clampedColors) {
      settings.nColors = clampedColors;
    }

    const clampedEmpty = Math.max(1, Math.min(3, Math.trunc(settings.nEmptyContainers || 1)));
    if (settings.nEmptyContainers !== clampedEmpty) {
      settings.nEmptyContainers = clampedEmpty;
    }

    const clampedVolume = Math.max(2, Math.min(20, Math.trunc(settings.containerVolume || 2)));
    if (settings.containerVolume !== clampedVolume) {
      settings.containerVolume = clampedVolume;
    }

    const clampedAutoNext = Math.max(0, Math.min(60, Math.trunc(autoNextSeconds || 0)));
    if (autoNextSeconds !== clampedAutoNext) {
      autoNextSeconds = clampedAutoNext;
    }
  });

  $effect(() => {
    const theme = appliedTheme;
    document.documentElement.dataset.theme = theme;
    document.body.dataset.theme = theme;
    const tokens = themeTokens[theme];
    document.documentElement.style.setProperty("--bg", tokens.bg);
    document.documentElement.style.setProperty("--surface", tokens.surface);
    document.documentElement.style.setProperty("--ink", tokens.ink);
    document.documentElement.style.setProperty("--muted", tokens.muted);
    document.documentElement.style.setProperty("--line", tokens.line);
    document.documentElement.style.setProperty("--cell-stroke", tokens.cellStroke);
    document.documentElement.style.colorScheme = theme;
    localStorage.setItem("sorted_theme_pref", themePref);
  });

  onMount(() => {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    systemDark = mq.matches;
    const listener = (ev: MediaQueryListEvent) => {
      systemDark = ev.matches;
    };
    mq.addEventListener("change", listener);

    const initialize = async () => {
      await loadUserOptionsConfig();

      // Local theme preference remains the final override for user interaction continuity.
      const savedTheme = localStorage.getItem("sorted_theme_pref") as ThemePref | null;
      if (savedTheme === "system" || savedTheme === "light" || savedTheme === "dark") {
        themePref = savedTheme;
      }

      loadHistory();
      await newRandomPuzzle();
    };

    void initialize();

    return () => {
      clearCountdown();
      clearGameTimer();
      mq.removeEventListener("change", listener);
    };
  });
</script>

<svelte:window onkeydown={onKeydown} />

<main id="app-main" class="page">
  <header id="app-header" class="topbar">
    <div id="header-brand">
      <h1 id="header-title">Sorted</h1>
    </div>
    <div id="header-actions" class="header-actions">
      <button id="button-theme-toggle" type="button" class="icon-button" onclick={toggleThemeQuick} aria-label="Toggle light or dark theme">
        {effectiveDark ? "☀" : "☾"}
      </button>
      <button id="button-toggle-options" type="button" class="icon-button" onclick={toggleOptionsOverlay} aria-expanded={showOptions} aria-controls="section-options">
        ☰
      </button>
    </div>
  </header>

  {#if showOptions}
    <div
      id="overlay-options"
      class="panel-overlay"
      role="button"
      tabindex="0"
      aria-label="Close options overlay"
      onclick={(ev) => {
        if (ev.target === ev.currentTarget) {
          showOptions = false;
        }
      }}
      onkeydown={(ev) => {
        if (ev.key === "Escape" || ev.key === "Enter" || ev.key === " ") {
          ev.preventDefault();
          showOptions = false;
        }
      }}
    >
      <section id="section-options" class="controls overlay-panel">
        <div id="group-colors-and-containers" class="control-group">
          <h2 id="heading-colors-and-containers">Colors and Containers</h2>
          <label for="input-n-colors">
            <span class="slider-label"><span>Colors</span><output for="input-n-colors">{settings.nColors}</output></span>
            <input id="input-n-colors" type="range" min="2" max="14" step="1" bind:value={settings.nColors} />
          </label>
          <label for="input-empty-containers">
            <span class="slider-label"><span>Empty Containers</span><output for="input-empty-containers">{settings.nEmptyContainers}</output></span>
            <input id="input-empty-containers" type="range" min="1" max="3" step="1" bind:value={settings.nEmptyContainers} />
          </label>
          <p id="text-total-containers" class="meta-text">Total Containers: {totalContainers}</p>
          <label for="input-container-volume">
            <span class="slider-label"><span>Container Volume</span><output for="input-container-volume">{settings.containerVolume}</output></span>
            <input id="input-container-volume" type="range" min="2" max="20" step="1" bind:value={settings.containerVolume} />
          </label>
          <label for="input-single-mode" class="check">
            <input id="input-single-mode" type="checkbox" bind:checked={singleMode} />
            Single Block Mode
          </label>
          <label for="theme-select">
            Theme
            <select id="theme-select" bind:value={themePref}>
              <option value="system">System</option>
              <option value="light">Light</option>
              <option value="dark">Dark</option>
            </select>
          </label>
        </div>

        <div id="group-puzzles" class="control-group">
          <h2 id="heading-puzzles">Puzzles</h2>
          <label for="input-auto-next-seconds">
            <span class="slider-label">
              <span>Auto next (sec): {autoNextSeconds}</span>
              <span class="slider-meta">
                <label for="input-auto-next-enabled" class="check inline-check">
                  <input id="input-auto-next-enabled" type="checkbox" bind:checked={autoNextEnabled} />
                  Auto next
                </label>
              </span>
            </span>
            <input id="input-auto-next-seconds" type="range" min="0" max="60" step="1" bind:value={autoNextSeconds} disabled={!autoNextEnabled} />
          </label>
          <button id="button-random-puzzle" type="button" onclick={newRandomPuzzle}>Generate Random Puzzle</button>
          <label for="input-puzzle-seed" class="seed-input">
            Puzzle Seed
            <span id="seed-input-row" class="seed-input-row">
              <input id="input-puzzle-seed" type="text" placeholder="16 hex chars (e.g. 0A1B2C3D4E5F6789)" bind:value={seedInput} />
              <button id="button-copy-seed" type="button" class="icon-button" onclick={copyCurrentSeed} disabled={!currentSeed} aria-label="Copy current puzzle seed" title="Copy current puzzle seed">
                <svg viewBox="0 0 17 22.25" aria-hidden="true" class="button-icon"><path d="M16.5,2.87v9.27c0,1.31-1.06,2.37-2.37,2.37h-3.63v-5.63c0-1.31-1.06-2.37-2.37-2.37h-1.63v-3.63c0-1.31,1.06-2.37,2.37-2.37h5.27c1.31,0,2.37,1.06,2.37,2.37ZM.5,8.87v9.27c0,1.31,1.06,2.37,2.37,2.37h5.27c1.31,0,2.37-1.06,2.37-2.37v-9.27c0-1.31-1.06-2.37-2.37-2.37H2.87c-1.31,0-2.37,1.06-2.37,2.37Z"/></svg>
              </button>
              <button id="button-paste-seed" type="button" class="icon-button" onclick={pasteSeedFromClipboard} aria-label="Paste puzzle seed" title="Paste puzzle seed">
                <svg viewBox="0 0 17 22.25" aria-hidden="true" class="button-icon"><path d="M5.5,17.75h-2.23c-1.53,0-2.77-1.24-2.77-2.77V4.52C.5,2.99,1.74,1.75,3.27,1.75h.48v-.46c0-.44.31-.79.7-.79h4.11c.38,0,.7.35.7.79v.46h.48c1.53,0,2.77,1.24,2.77,2.77v2.23M3.75,1.75v.46c0,.44.31.79.7.79h4.11c.38,0,.7-.35.7-.79v-.46M7.87,6.75h5.27c1.31,0,2.37,1.06,2.37,2.37v9.27c0,1.31-1.06,2.37-2.37,2.37h-5.27c-1.31,0-2.37-1.06-2.37-2.37v-9.27c0-1.31,1.06-2.37,2.37-2.37Z"/><line x1="9" y1="17.25" x2="12" y2="17.25"/><line x1="9" y1="13.75" x2="12" y2="13.75"/><line x1="9" y1="10.25" x2="12" y2="10.25"/></svg>
              </button>
              <button id="button-play-seed" type="button" class="icon-button" onclick={playSeedPuzzle} aria-label="Play puzzle seed" title="Play puzzle seed">
                <svg viewBox="0 0 24 24" aria-hidden="true" class="button-icon fill-icon"><path d="M8 6v12l10-6Z"></path></svg>
              </button>
            </span>
          </label>
          <p id="current-seed" class="meta-text">Current Puzzle Seed: <span class="seed-text">{currentPuzzleSeed}</span></p>
          <button id="button-toggle-history" type="button" onclick={toggleHistoryOverlay}>
            {showHistory ? "Hide" : "Show"} Past Puzzles
          </button>
        </div>
      </section>
    </div>
  {/if}

  <section id="game-status" class="status">
    <p id="current-status">{statusLine}</p>
    <p id="move-count">Move Count: {userMoves}</p>
    <p id="total-moves">Total Moves: {totalMoves}</p>
    <p id="game-timer">Time: {formatDuration(elapsedSeconds)}</p>
  </section>

  <section id="game-board-wrapper" class="board-wrap" bind:clientWidth={boardViewportWidth}>
    <div id="game-board" class="board" style={`--containers:${board.containers.length}; --container-volume:${board.containerVolume};`}>
      {#each boardRows as row}
        <div class="board-row" style={`--row-containers:${row.length};`}>
          {#each row as entry}
            <button
              id={`container-${entry.index + 1}`}
              class:selected={selectedSrc === entry.index}
              class="container"
              type="button"
              onclick={() => onContainerClick(entry.index)}
              aria-label={`Container ${entry.index + 1}`}
            >
              <span class="key-badge">{keyLabel(entry.index)}</span>
              {#each Array(board.containerVolume) as _, rowIndex}
                {@const value = entry.container[board.containerVolume - 1 - rowIndex] ?? 0}
                <span
                  class="cell"
                  style={`background:${palette[value] ?? "#fff"}`}
                ></span>
              {/each}
            </button>
          {/each}
        </div>
      {/each}
    </div>
  </section>

  <div id="section-action-buttons" class="action-buttons" style={`--containers:${board.containers.length};`}>
    <button id="button-undo-move" type="button" class="action-icon-button" onclick={undoMove} disabled={moveHistory.length === 0} aria-label="Undo move" title="Undo move">↶</button>
    <button id="button-redo-move" type="button" class="action-icon-button" onclick={redoMove} disabled={redoHistory.length === 0} aria-label="Redo move" title="Redo move">↷</button>
    <button id="button-restart-puzzle" type="button" class="action-icon-button" onclick={confirmRestartCurrentPuzzle} disabled={!initialState} aria-label="Restart puzzle" title="Restart puzzle">⟲</button>
  </div>

  {#if overlayOpen}
    <section id="end-state-overlay" class="overlay">
      <h2>{overlayStatus === "solved" ? "Puzzle solved" : "No more moves"}</h2>
      <p>Move Count: {userMoves}</p>
      <p>Total Moves: {totalMoves}</p>
      <p>Time: {formatDuration(elapsedSeconds)}</p>
      {#if overlayStatus === "solved"}
        {#if autoNextEnabled}
          <p>Next random puzzle in {countdown}s. Press Space or Enter to load now.</p>
        {:else}
          <p>Press Space or Enter to load the next puzzle.</p>
        {/if}
      {:else}
        <p>No auto-next while stuck. Use undo/restart/random/history/seed.</p>
      {/if}
      <div id="overlay-actions" class="overlay-actions">
        {#if overlayStatus === "solved"}
          <button id="button-overlay-next" type="button" onclick={newRandomPuzzle}>Start Next Now</button>
        {:else}
          <button id="button-overlay-undo" type="button" onclick={() => { overlayOpen = false; undoMove(); }}>Undo Move</button>
          <button id="button-overlay-restart" type="button" onclick={confirmRestartCurrentPuzzle}>Restart Current Puzzle</button>
          <button id="button-overlay-random" type="button" onclick={newRandomPuzzle}>Random Puzzle</button>
        {/if}
        <button id="button-overlay-history" type="button" onclick={() => (showHistory = true)}>Open Past Puzzles</button>
      </div>
    </section>
  {/if}

  {#if showHistory}
    <div
      id="overlay-history"
      class="panel-overlay"
      role="button"
      tabindex="0"
      aria-label="Close history overlay"
      onclick={(ev) => {
        if (ev.target === ev.currentTarget) {
          showHistory = false;
        }
      }}
      onkeydown={(ev) => {
        if (ev.key === "Escape" || ev.key === "Enter" || ev.key === " ") {
          ev.preventDefault();
          showHistory = false;
        }
      }}
    >
      <section id="past-puzzles" class="history overlay-panel">
        <div id="past-puzzles-header" class="past-puzzles-header">
          <h2>Past Puzzles</h2>
          <button id="button-clear-history" type="button" onclick={clearHistory}>Clear History</button>
        </div>
        {#if pastPuzzles.length === 0}
          <p>No finished puzzles yet.</p>
        {:else}
          <!-- TODO: Re-add PB columns after implementing stable historical PB recalculation. -->
          <table>
            <thead>
              <tr>
                <th>Date</th>
                <th>Result</th>
                <th>Move Count</th>
                <th>Total Moves</th>
                <th>Time</th>
                <th>Seed</th>
                <th>Settings</th>
                <th></th>
              </tr>
            </thead>
            <tbody>
              {#each pastPuzzles as puzzle}
                <tr>
                  <td>{formatDateTime(puzzle.finishedAt)}</td>
                  <td>{puzzle.endedAs}</td>
                  <td>{puzzle.userMoves}</td>
                  <td>{puzzle.totalMoves}</td>
                  <td>{formatDuration(puzzle.elapsedSeconds)}</td>
                  <td>{puzzle.puzzleSeed || "n/a"}</td>
                  <td>{puzzle.settings.nColors}/{puzzle.settings.nColors + puzzle.settings.nEmptyContainers}/{puzzle.settings.containerVolume}</td>
                  <td>
                    <button id={`button-play-again-${puzzle.id}`} type="button" onclick={() => playAgain(puzzle)}>Play Again</button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        {/if}
      </section>
    </div>
  {/if}

</main>

<style>
  :global(body) {
    margin: 0;
    background: var(--bg);
    color: var(--ink);
    font-family: "Aptos", "Manrope", "Segoe UI Variable", sans-serif;
  }

  .page {
    padding: 1rem;
    max-width: 1480px;
    margin: 0 auto;
    display: grid;
    gap: 1rem;
  }

  h1,
  h2,
  p {
    margin: 0;
  }

  .topbar {
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    gap: 1rem;
  }

  #header-brand {
    grid-column: 2;
    justify-self: center;
    text-align: center;
  }

  .header-actions {
    grid-column: 3;
    display: flex;
    gap: 0.5rem;
    align-items: center;
    justify-self: end;
  }

  .icon-button {
    min-width: 2.2rem;
    min-height: 2.2rem;
    font-size: 1.05rem;
  }

  .controls {
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: 10px;
    padding: 0.75rem;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
    gap: 0.75rem;
    align-items: start;
  }

  .panel-overlay {
    position: fixed;
    inset: 0;
    background: rgba(8, 10, 14, 0.6);
    backdrop-filter: blur(3px);
    display: grid;
    place-items: center;
    z-index: 120;
    padding: 1rem;
  }

  .overlay-panel {
    width: min(1100px, calc(100vw - 2rem));
    max-height: calc(100vh - 2rem);
    overflow: auto;
  }

  .control-group {
    display: grid;
    gap: 0.55rem;
    border: 1px solid var(--line);
    border-radius: 8px;
    padding: 0.65rem;
  }

  .control-group h2 {
    font-size: 1rem;
  }

  .meta-text {
    color: var(--muted);
    font-size: 0.9rem;
  }

  .slider-label {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.75rem;
  }

  .slider-meta {
    display: inline-flex;
    align-items: center;
    gap: 0.75rem;
  }

  label {
    display: grid;
    gap: 0.2rem;
    font-size: 0.9rem;
  }

  .check {
    display: inline-flex;
    gap: 0.5rem;
    align-items: center;
  }

  .inline-check {
    white-space: nowrap;
  }

  .seed-input input {
    width: 100%;
    min-width: 0;
  }

  .seed-input-row {
    display: flex;
    gap: 0.45rem;
    align-items: center;
  }

  .seed-input-row > input {
    flex: 1 1 auto;
    min-width: 0;
  }

  .seed-input-row > .icon-button {
    flex: 0 0 2.8rem;
    width: 2.8rem;
    min-width: 2.8rem;
    min-height: 2.2rem;
    padding: 0;
    display: inline-grid;
    place-items: center;
  }

  input,
  select,
  button {
    border: 1px solid var(--line);
    border-radius: 8px;
    background: var(--surface);
    color: var(--ink);
    padding: 0.4rem 0.55rem;
  }

  select {
    appearance: none;
    background-color: var(--surface);
  }

  input[type="range"] {
    padding: 0;
    accent-color: #4b86ff;
  }

  button {
    cursor: pointer;
  }

  .button-icon {
    width: 1.35rem;
    height: 1.35rem;
    stroke: currentColor;
    stroke-width: 1.9;
    stroke-linecap: round;
    stroke-linejoin: round;
    stroke-miterlimit: 10;
    fill: none;
    display: block;
  }

  .fill-icon {
    fill: currentColor;
    stroke: none;
  }
/*
  .icon-with-label {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
  }
 */

  .board-wrap {
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: 10px;
    padding: 1.5rem;
  }

  .board {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .board-row {
    display: flex;
    justify-content: center;
    gap: 1rem;
    width: 100%;
  }

  .container {
    position: relative;
    display: grid;
    grid-auto-rows: 30px;
    gap: 4px;
    padding: 1.55rem 0.38rem 0.4rem;
    border: 2px solid var(--line);
    border-radius: 0 0 10px 10px;
    min-height: 164px;
    width: clamp(92px, 6.8vw, 112px);
  }

  .container.selected {
    border-color: #ea4f4f;
  }

  .key-badge {
    position: absolute;
    top: -0.78rem;
    left: 50%;
    transform: translateX(-50%);
    font-size: 0.8rem;
    border: 1px solid var(--line);
    border-radius: 999px;
    padding: 0.08rem 0.42rem;
    background: var(--surface);
  }

  .cell {
    border-radius: 2px;
    border: 1px solid var(--cell-stroke);
  }

  .history,
  .overlay {
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: 10px;
    padding: 0.75rem;
    display: grid;
    gap: 0.35rem;
  }

  .status {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0;
    flex-wrap: wrap;
  }

  .seed-text {
    display: inline-block;
    max-width: 100%;
    word-break: break-all;
  }

  .action-buttons {
    display: flex;
    justify-content: center;
    gap: 0.6rem;
    flex-wrap: wrap;
    margin-top: 0.9rem;
  }

  .action-icon-button {
    width: min(72px, max(52px, calc((100% - (var(--containers, 1) - 1) * 0.65rem) / var(--containers, 1))));
    aspect-ratio: 1;
    display: inline-grid;
    place-items: center;
    padding: 0;
    font-size: 2.1rem;
    line-height: 1;
    font-weight: 700;
  }

  .past-puzzles-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .overlay-actions {
    display: flex;
    gap: 0.6rem;
    flex-wrap: wrap;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.9rem;
  }

  th,
  td {
    border-bottom: 1px solid var(--line);
    text-align: left;
    padding: 0.45rem;
  }

  @media (max-width: 720px) {
    .topbar {
      grid-template-columns: 1fr;
      justify-items: stretch;
    }

    #header-brand,
    .header-actions {
      grid-column: auto;
      justify-self: start;
      text-align: left;
    }

    .status {
      align-items: flex-start;
    }

    .slider-label {
      align-items: flex-start;
      flex-direction: column;
    }
  }
</style>
