<script lang="ts">
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

  type GameAction =
    | { kind: "pour"; move: Move }
    | { kind: "bonusSection" };

  type BonusStage = 0 | 1 | 2;

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

  type UserOptionsConfig = {
    theme?: ThemePref;
    nColors?: number;
    containerVolume?: number;
    singleBlockMode?: boolean;
    autoNextEnabled?: boolean;
    autoNextSeconds?: number;
    showColumnKeys?: boolean;
  };

  const FALLBACK_KEYS = "1234567890QWERTYUIO";
  const HISTORY_KEY = "sorted_history_v1";
  const SHOW_COLUMN_KEYS_KEY = "sorted_show_column_keys";
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
  const BOARD_ITEM_TARGET_WIDTH = 36;
  const BOARD_GAP_PX = 4;

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

  let settings = $state<PuzzleSettings>({ nColors: 12, nEmptyContainers: 2, containerVolume: 4 });
  let singleMode = $state(false);
  let autoNextEnabled = $state(true);
  let autoNextSeconds = $state(12);
  let showHistory = $state(false);
  let showOptions = $state(false);
  let showColumnKeys = $state(true);
  let hasColumnKeyPreference = $state(false);
  let coarsePointer = $state(false);

  let board = $state<PuzzleState>({ containers: [], containerVolume: 4 });
  let initialState = $state<PuzzleState | null>(null);
  let moveHistory = $state<GameAction[]>([]);
  let redoHistory = $state<GameAction[]>([]);
  let containerCapacities = $state<number[]>([]);
  let initialContainerCapacities = $state<number[]>([]);
  let bonusContainerStage = $state<BonusStage>(0);
  let initialBonusContainerStage = $state<BonusStage>(0);
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
  let viewportWidth = $state(0);
  let viewportHeight = $state(0);

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
  const effectiveEmptyContainers = $derived(settings.nEmptyContainers + bonusContainerStage * 0.5);
  const totalContainers = $derived(settings.nColors + effectiveEmptyContainers);
  const isMobileLayout = $derived(viewportWidth > 0 && viewportWidth <= 720);
  const forceSingleBoardRow = $derived(!showColumnKeys && board.containerVolume >= 10);
  const boardRowCounts = $derived.by(() => {
    if (forceSingleBoardRow) {
      return [board.containers.length];
    }
    return pickBoardLayout(board.containers.length, boardViewportWidth);
  });
  const boardWidestRow = $derived.by(() => {
    if (boardRowCounts.length === 0) {
      return Math.max(board.containers.length, 1);
    }
    return Math.max(...boardRowCounts);
  });
  const boardCellSize = $derived.by(() => computeBoardCellSize({
    availableWidth: boardViewportWidth,
    availableHeight: viewportHeight,
    volume: board.containerVolume || settings.containerVolume,
    rowCount: Math.max(boardRowCounts.length, 1),
    widestRow: Math.max(boardWidestRow, 1),
    compact: isMobileLayout,
    showColumnKeys
  }));
  const boardStyle = $derived.by(() => {
    const boardGap = isMobileLayout ? "0.18rem" : "0.35rem";
    const rowGap = isMobileLayout ? "0.45rem" : "0.7rem";
    const cellGap = isMobileLayout ? "1px" : "2px";
    const sidePad = isMobileLayout ? "1px" : "2px";
    const bottomPad = isMobileLayout ? "1px" : "2px";
    const topPad = showColumnKeys
      ? (isMobileLayout ? "0.72rem" : "0.82rem")
      : (isMobileLayout ? "0.16rem" : "0.28rem");
    return `--cell-size:${boardCellSize}px; --board-gap:${boardGap}; --row-gap:${rowGap}; --cell-gap:${cellGap}; --container-side-pad:${sidePad}; --container-bottom-pad:${bottomPad}; --container-top-pad:${topPad};`;
  });
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

  function cloneState(state: PuzzleState): PuzzleState {
    return {
      containerVolume: state.containerVolume,
      containers: state.containers.map((v) => [...v])
    };
  }

  function defaultContainerCapacities(state: PuzzleState): number[] {
    return Array.from({ length: state.containers.length }, () => state.containerVolume);
  }

  function containerCapacityFor(state: PuzzleState, capacities: number[], index: number): number {
    return capacities[index] ?? state.containerVolume;
  }

  function applyBonusSectionToState(
    state: PuzzleState,
    capacities: number[],
    stage: BonusStage
  ): { state: PuzzleState; capacities: number[]; stage: BonusStage } | null {
    if (stage === 2) {
      return null;
    }

    const nextState = cloneState(state);
    const nextCaps = [...capacities];

    if (stage === 0) {
      const halfVolume = Math.max(1, Math.floor(state.containerVolume / 2));
      nextState.containers.push([]);
      nextCaps.push(halfVolume);
      return { state: nextState, capacities: nextCaps, stage: 1 };
    }

    const index = nextState.containers.length - 1;
    if (index < 0) {
      return null;
    }
    nextCaps[index] = state.containerVolume;
    return { state: nextState, capacities: nextCaps, stage: 2 };
  }

  function applyActionToState(
    state: PuzzleState,
    capacities: number[],
    stage: BonusStage,
    action: GameAction
  ): { state: PuzzleState; capacities: number[]; stage: BonusStage } | null {
    if (action.kind === "bonusSection") {
      return applyBonusSectionToState(state, capacities, stage);
    }

    const next = applyMove(state, action.move, capacities);
    if (!next) {
      return null;
    }
    return { state: next, capacities: [...capacities], stage };
  }

  function rebuildFromActionHistory(actions: GameAction[]): {
    state: PuzzleState;
    capacities: number[];
    stage: BonusStage;
  } | null {
    if (!initialState) {
      return null;
    }

    let rebuiltState = cloneState(initialState);
    let rebuiltCaps = [...initialContainerCapacities];
    let rebuiltStage = initialBonusContainerStage;

    for (const action of actions) {
      const applied = applyActionToState(rebuiltState, rebuiltCaps, rebuiltStage, action);
      if (!applied) {
        return null;
      }
      rebuiltState = applied.state;
      rebuiltCaps = applied.capacities;
      rebuiltStage = applied.stage;
    }

    return { state: rebuiltState, capacities: rebuiltCaps, stage: rebuiltStage };
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

  function computeBoardCellSize(input: {
    availableWidth: number;
    availableHeight: number;
    volume: number;
    rowCount: number;
    widestRow: number;
    compact: boolean;
    showColumnKeys: boolean;
  }): number {
    const volume = Math.max(1, input.volume || 1);
    const widestRow = Math.max(1, input.widestRow || 1);
    const rowCount = Math.max(1, input.rowCount || 1);
    const borderWidth = 2;
    const boardGap = input.compact ? 3 : 6;
    const rowGap = input.compact ? 7 : 11;
    const cellGap = input.compact ? 1 : 2;
    const sidePad = input.compact ? 1 : 2;
    const bottomPad = input.compact ? 1 : 2;
    const topPad = input.showColumnKeys
      ? (input.compact ? 12 : 14)
      : (input.compact ? 3 : 5);
    const widthAvailable = Math.max(140, input.availableWidth || 320);
    const heightAvailable = Math.max(280, input.availableHeight || 640);
    const reservedHeight = input.compact ? 250 : 310;
    const boardHeightBudget = Math.max(120, heightAvailable - reservedHeight);
    const widthLimited = (
      widthAvailable - (widestRow - 1) * boardGap - widestRow * (sidePad * 2 + borderWidth * 2)
    ) / widestRow;
    const heightLimited = (
      boardHeightBudget - (rowCount - 1) * rowGap - rowCount * (topPad + bottomPad + borderWidth * 2) - rowCount * (volume - 1) * cellGap
    ) / (rowCount * volume);
    const minSize = input.compact ? 12 : 18;
    const maxSize = input.compact ? 42 : 56;
    return Math.max(minSize, Math.min(maxSize, Math.floor(Math.min(widthLimited, heightLimited))));
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

  function isSolvedState(state: PuzzleState, capacities: number[]): boolean {
    for (let i = 0; i < state.containers.length; i += 1) {
      const container = state.containers[i];
      const capacity = containerCapacityFor(state, capacities, i);
      if (container.length === 0) {
        continue;
      }
      if (container.length !== capacity) {
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

  function availableMoveCount(state: PuzzleState, capacities: number[]): number {
    let count = 0;
    for (let src = 0; src < state.containers.length; src += 1) {
      for (let dst = 0; dst < state.containers.length; dst += 1) {
        if (canApplyMove(state, src, dst, capacities)) {
          count += 1;
        }
      }
    }
    return count;
  }

  function fallbackCheckState(state: PuzzleState, capacities: number[]): StateCheck {
    if (isSolvedState(state, capacities)) {
      return { status: "solved", movesAvailable: 0 };
    }

    const movesAvailable = availableMoveCount(state, capacities);
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

  async function checkState(state: PuzzleState, capacities: number[]): Promise<StateCheck> {
    void singleMode;
    return fallbackCheckState(state, capacities);
  }

  function canApplyMove(state: PuzzleState, src: number, dst: number, capacities: number[]): boolean {
    if (src === dst) {
      return false;
    }
    const srcContainer = state.containers[src];
    const dstContainer = state.containers[dst];
    const dstCapacity = containerCapacityFor(state, capacities, dst);
    if (!srcContainer || !dstContainer || srcContainer.length === 0 || dstContainer.length >= dstCapacity) {
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

  function applyMove(state: PuzzleState, move: Move, capacities: number[]): PuzzleState | null {
    if (!canApplyMove(state, move.src, move.dst, capacities)) {
      return null;
    }

    const next = cloneState(state);
    const srcContainer = next.containers[move.src];
    const dstContainer = next.containers[move.dst];
    const srcTop = topRun(srcContainer);
    if (!srcTop) {
      return null;
    }

    const dstCapacity = containerCapacityFor(state, capacities, move.dst);
    const amount = singleMode ? 1 : Math.min(srcTop.run, dstCapacity - dstContainer.length);
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

  function formatEmptyContainers(value: number): string {
    return Number.isInteger(value) ? value.toString() : value.toFixed(1);
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

    const configuredVolume = toFiniteNumber(config.containerVolume);
    if (configuredVolume !== null) {
      settings.containerVolume = clampInteger(configuredVolume, 2, 20);
    }

    if (typeof config.showColumnKeys === "boolean") {
      showColumnKeys = config.showColumnKeys;
      hasColumnKeyPreference = true;
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
          nColors: entry.settings?.nColors ?? 12,
          nEmptyContainers:
            entry.settings?.nEmptyContainers ??
            ((entry.settings?.nContainers ?? (entry.settings?.nColors ?? 12) + 2) - (entry.settings?.nColors ?? 12)),
          containerVolume: entry.settings?.containerVolume ?? entry.settings?.nVolume ?? 4
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
    settings.nEmptyContainers = 2;
    moveHistory = [];
    redoHistory = [];
    bonusContainerStage = 0;
    initialBonusContainerStage = 0;
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
    containerCapacities = defaultContainerCapacities(board);
    initialContainerCapacities = [...containerCapacities];
    statusLine = "Puzzle loaded. Pick source then target container.";
  }

  function confirmNewRandomPuzzle(): void {
    if (window.confirm("Generate a new random puzzle? Current progress will be lost.")) {
      void newRandomPuzzle();
    }
  }

  async function addContainerSection(): Promise<void> {
    if (bonusContainerStage >= 2 || !initialState) {
      return;
    }

    const applied = applyBonusSectionToState(board, containerCapacities, bonusContainerStage);
    if (!applied) {
      statusLine = "Could not add container section.";
      return;
    }

    if (!gameTimerIntervalId) {
      startGameTimer();
    }

    board = applied.state;
    containerCapacities = applied.capacities;
    bonusContainerStage = applied.stage;
    moveHistory = [...moveHistory, { kind: "bonusSection" }];
    redoHistory = [];
    userMoves += 1;
    totalMoves += 1;
    selectedSrc = null;

    statusLine = bonusContainerStage === 1
      ? "Added half empty container section. Press + once more to finalize."
      : "Upgraded bonus container to full capacity.";

    const check = await checkState(board, containerCapacities);
    if (check.status === "solved" || check.status === "stuck") {
      await openStats(check.status);
    }
  }

  async function applyUserMove(move: Move): Promise<void> {
    const updated = applyMove(board, move, containerCapacities);
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
    moveHistory = [...moveHistory, { kind: "pour", move }];
    redoHistory = [];
    userMoves += 1;
    totalMoves += 1;
    statusLine = `Move ${userMoves}: ${formatMove(move)}`;

    const check = await checkState(board, containerCapacities);
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
        await redoMove();
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
      await redoMove();
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
    const rebuilt = rebuildFromActionHistory(nextHistory);
    if (!rebuilt) {
      statusLine = "Undo failed due to inconsistent state.";
      return;
    }

    board = rebuilt.state;
    containerCapacities = rebuilt.capacities;
    bonusContainerStage = rebuilt.stage;
    const undoneMove = moveHistory[moveHistory.length - 1];
    moveHistory = nextHistory;
    redoHistory = undoneMove ? [...redoHistory, undoneMove] : redoHistory;
    userMoves = nextHistory.length;
    totalMoves += 1;
    selectedSrc = null;

    // Undoing from a stuck end state should return the player to active play.
    if (overlayOpen && overlayStatus === "stuck") {
      overlayOpen = false;
    }

    statusLine = "Undid last move.";
  }

  async function redoMove(): Promise<void> {
    if (redoHistory.length === 0) {
      return;
    }

    const action = redoHistory[redoHistory.length - 1];
    const applied = applyActionToState(board, containerCapacities, bonusContainerStage, action);
    if (!applied) {
      statusLine = "Redo failed.";
      return;
    }

    board = applied.state;
    containerCapacities = applied.capacities;
    bonusContainerStage = applied.stage;
    moveHistory = [...moveHistory, action];
    redoHistory = redoHistory.slice(0, -1);
    userMoves = moveHistory.length;
    totalMoves += 1;
    selectedSrc = null;
    statusLine = action.kind === "pour"
      ? `Redid move: ${formatMove(action.move)}`
      : "Redid bonus container section.";

    const check = await checkState(board, containerCapacities);
    if (check.status === "stuck") {
      showEndOverlay("stuck", false);
    }
  }

  function restartCurrentPuzzle(): void {
    if (!initialState) {
      return;
    }

    clearCountdown();
    clearGameTimer();
    overlayOpen = false;
    board = cloneState(initialState);
    containerCapacities = [...initialContainerCapacities];
    bonusContainerStage = initialBonusContainerStage;
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
    settings.nEmptyContainers = 2;
    board = fromBackendState(seeded);
    initialState = cloneState(fromBackendState(seeded));
    containerCapacities = defaultContainerCapacities(board);
    initialContainerCapacities = [...containerCapacities];
    bonusContainerStage = 0;
    initialBonusContainerStage = 0;
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
      settings: {
        nColors: settings.nColors,
        containerVolume: settings.containerVolume,
        nEmptyContainers: effectiveEmptyContainers
      },
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

  function showEndOverlay(endedAs: "solved" | "stuck", recordHistory: boolean): void {
    clearGameTimer();
    overlayOpen = true;
    overlayStatus = endedAs;
    if (recordHistory) {
      addPastPuzzle({ endedAs });
    }
    if (endedAs === "solved") {
      startCountdown();
    } else {
      clearCountdown();
    }
  }

  async function openStats(endedAs: "solved" | "stuck"): Promise<void> {
    showEndOverlay(endedAs, true);
  }

  function playAgain(entry: PastPuzzle): void {
    clearCountdown();
    clearGameTimer();
    settings = {
      nColors: entry.settings.nColors,
      containerVolume: entry.settings.containerVolume,
      nEmptyContainers: 2
    };
    singleMode = entry.singleMode;
    let loadedBase = cloneState(entry.initialState);
    let loadedCaps = defaultContainerCapacities(loadedBase);
    let loadedStage: BonusStage = 0;
    const targetStage: BonusStage = entry.settings.nEmptyContainers >= 3
      ? 2
      : (entry.settings.nEmptyContainers >= 2.5 ? 1 : 0);
    while (loadedStage < targetStage) {
      const applied = applyBonusSectionToState(loadedBase, loadedCaps, loadedStage);
      if (!applied) {
        break;
      }
      loadedBase = applied.state;
      loadedCaps = applied.capacities;
      loadedStage = applied.stage;
    }

    board = loadedBase;
    initialState = cloneState(loadedBase);
    containerCapacities = loadedCaps;
    initialContainerCapacities = [...loadedCaps];
    bonusContainerStage = loadedStage;
    initialBonusContainerStage = loadedStage;
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

    const clampedEmpty = 2;
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

  $effect(() => {
    if (!hasColumnKeyPreference) {
      return;
    }
    localStorage.setItem(SHOW_COLUMN_KEYS_KEY, showColumnKeys ? "true" : "false");
  });

  onMount(() => {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const coarsePointerMq = window.matchMedia("(pointer: coarse)");
    systemDark = mq.matches;
    coarsePointer = coarsePointerMq.matches;
    const listener = (ev: MediaQueryListEvent) => {
      systemDark = ev.matches;
    };
    const coarsePointerListener = (ev: MediaQueryListEvent) => {
      coarsePointer = ev.matches;
      if (!hasColumnKeyPreference) {
        showColumnKeys = !ev.matches;
      }
    };
    mq.addEventListener("change", listener);
    coarsePointerMq.addEventListener("change", coarsePointerListener);

    const initialize = async () => {
      await loadUserOptionsConfig();

      // Local theme preference remains the final override for user interaction continuity.
      const savedTheme = localStorage.getItem("sorted_theme_pref") as ThemePref | null;
      if (savedTheme === "system" || savedTheme === "light" || savedTheme === "dark") {
        themePref = savedTheme;
      }

      const savedColumnKeys = localStorage.getItem(SHOW_COLUMN_KEYS_KEY);
      if (savedColumnKeys === "true" || savedColumnKeys === "false") {
        showColumnKeys = savedColumnKeys === "true";
        hasColumnKeyPreference = true;
      } else {
        showColumnKeys = !coarsePointer;
      }

      loadHistory();
      await newRandomPuzzle();
    };

    void initialize();

    return () => {
      clearCountdown();
      clearGameTimer();
      mq.removeEventListener("change", listener);
      coarsePointerMq.removeEventListener("change", coarsePointerListener);
    };
  });
</script>

<svelte:window bind:innerWidth={viewportWidth} bind:innerHeight={viewportHeight} onkeydown={onKeydown} />

<main id="app-main" class="page">
  <header id="app-header" class="topbar">
    <div id="header-brand">
      <h1 id="header-title">Sorted</h1>
    </div>
    <div id="header-actions" class="header-actions">
      <button id="button-theme-toggle" type="button" class="icon-button" onclick={toggleThemeQuick} aria-label="Toggle light or dark theme">
        {#if effectiveDark}
          <svg viewBox="0 0 24 24" aria-hidden="true" class="button-icon"><use href="/icons/icon-theme-light.svg#icon-theme-light"/></svg>
        {:else}
          <svg viewBox="0 0 24 24" aria-hidden="true" class="button-icon fill-icon"><use href="/icons/icon-theme-dark.svg#icon-theme-dark"/></svg>
        {/if}
      </button>
      <button id="button-toggle-options" type="button" class="icon-button" onclick={toggleOptionsOverlay} aria-expanded={showOptions} aria-controls="section-options" aria-label="Toggle options menu" title="Toggle options menu">
        <svg viewBox="0 0 24 24" aria-hidden="true" class="button-icon"><use href="/icons/icon-menu.svg#icon-menu"/></svg>
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
          <p id="text-empty-containers" class="meta-text">Empty Containers: {formatEmptyContainers(effectiveEmptyContainers)}</p>
          <p id="text-total-containers" class="meta-text">Total Containers: {formatEmptyContainers(totalContainers)}</p>
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
          <label for="input-show-column-keys" class="check">
            <input
              id="input-show-column-keys"
              type="checkbox"
              bind:checked={showColumnKeys}
              onchange={() => {
                hasColumnKeyPreference = true;
              }}
            />
            Show Column Keys
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
                <svg viewBox="0 0 17 22.25" aria-hidden="true" class="button-icon"><use href="/icons/icon-copy-seed.svg#icon-copy-seed"/></svg>
              </button>
              <button id="button-paste-seed" type="button" class="icon-button" onclick={pasteSeedFromClipboard} aria-label="Paste puzzle seed" title="Paste puzzle seed">
                <svg viewBox="0 0 17 22.25" aria-hidden="true" class="button-icon"><use href="/icons/icon-paste-seed.svg#icon-paste-seed"/></svg>
              </button>
              <button id="button-play-seed" type="button" class="icon-button" onclick={playSeedPuzzle} aria-label="Play puzzle seed" title="Play puzzle seed">
                <svg viewBox="0 0 24 24" aria-hidden="true" class="button-icon fill-icon"><use href="/icons/icon-play-seed.svg#icon-play-seed"/></svg>
              </button>
            </span>
          </label>
          <p id="current-seed" class="meta-text">Current Puzzle Seed: <span class="seed-text">{currentPuzzleSeed}</span></p>
          <button id="button-toggle-history" type="button" class="icon-with-label" onclick={toggleHistoryOverlay}>
            <svg viewBox="0 0 24 24" class="button-icon"><use href="/icons/icon-show-past-puzzles.svg#icon-show-past-puzzles"/></svg>
            <span>{showHistory ? "Hide" : "Show"} Past Puzzles</span>
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
    <div id="game-board" class="board" class:single-row-volume={forceSingleBoardRow} style={`--containers:${board.containers.length}; --container-volume:${board.containerVolume}; ${boardStyle}`}>
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
              {#if showColumnKeys}
                <span class="key-badge">{keyLabel(entry.index)}</span>
              {/if}
              {#each Array(containerCapacities[entry.index] ?? board.containerVolume) as _, rowIndex}
                {@const containerCapacity = containerCapacities[entry.index] ?? board.containerVolume}
                {@const filledStartRow = containerCapacity - entry.container.length}
                {@const logicalFilledIndex = rowIndex - filledStartRow}
                {@const value = logicalFilledIndex < 0
                  ? 0
                  : (entry.container[entry.container.length - 1 - logicalFilledIndex] ?? 0)}
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
    <button id="button-undo-move" type="button" class="action-icon-button" onclick={undoMove} disabled={moveHistory.length === 0} aria-label="Undo move" title="Undo move">
      <svg viewBox="0 0 24 24" class="button-icon"><use href="/icons/icon-undo.svg#icon-undo"/></svg>
    </button>
    <button id="button-redo-move" type="button" class="action-icon-button" onclick={redoMove} disabled={redoHistory.length === 0} aria-label="Redo move" title="Redo move">
      <svg viewBox="0 0 24 24" class="button-icon"><use href="/icons/icon-redo.svg#icon-redo"/></svg>
    </button>
    <button id="button-restart-puzzle" type="button" class="action-icon-button" onclick={confirmRestartCurrentPuzzle} disabled={!initialState} aria-label="Restart puzzle" title="Restart puzzle">
      <svg viewBox="0 0 24 24" class="button-icon"><use href="/icons/icon-restart.svg#icon-restart"/></svg>
    </button>
    <button id="button-random-puzzle-inline" type="button" class="action-icon-button" onclick={confirmNewRandomPuzzle} aria-label="New puzzle" title="Generate a new puzzle">
      <svg viewBox="0 0 24 24" class="button-icon"><use href="/icons/icon-random-puzzle.svg#icon-random-puzzle"/></svg>
    </button>
    <button id="button-add-container-section" type="button" class="action-icon-button" onclick={addContainerSection} disabled={!initialState || bonusContainerStage >= 2} aria-label="Add container section" title="Add empty container section">
      <svg viewBox="0 0 24 24" class="button-icon"><use href="/icons/icon-add-container-section.svg#icon-add-container-section"/></svg>
    </button>
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
          <button id="button-overlay-next" type="button" class="icon-with-label" onclick={newRandomPuzzle}>
            <svg viewBox="0 0 24 24" class="button-icon fill-icon"><use href="/icons/icon-play-seed.svg#icon-play-seed"/></svg>
            <span>Start Next Now</span>
          </button>
        {:else}
          <button id="button-overlay-undo" type="button" class="icon-with-label" onclick={() => { overlayOpen = false; undoMove(); }}>
            <svg viewBox="0 0 24 24" class="button-icon"><use href="/icons/icon-undo.svg#icon-undo"/></svg>
            <span>Undo Move</span>
          </button>
          <button id="button-overlay-restart" type="button" class="icon-with-label" onclick={confirmRestartCurrentPuzzle}>
            <svg viewBox="0 0 24 24" class="button-icon"><use href="/icons/icon-restart.svg#icon-restart"/></svg>
            <span>Restart Current Puzzle</span>
          </button>
          <button id="button-overlay-random" type="button" class="icon-with-label" onclick={newRandomPuzzle}>
            <svg viewBox="0 0 24 24" class="button-icon"><use href="/icons/icon-random-puzzle.svg#icon-random-puzzle"/></svg>
            <span>Random Puzzle</span>
          </button>
        {/if}
        <button id="button-overlay-history" type="button" class="icon-with-label" onclick={() => (showHistory = true)}>
          <svg viewBox="0 0 24 24" class="button-icon"><use href="/icons/icon-show-past-puzzles.svg#icon-show-past-puzzles"/></svg>
          <span>Open Past Puzzles</span>
        </button>
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
          <button id="button-clear-history" type="button" class="icon-with-label" onclick={clearHistory}>
            <svg viewBox="0 0 24 24" class="button-icon"><use href="/icons/icon-clear-history.svg#icon-clear-history"/></svg>
            <span>Clear History</span>
          </button>
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
                  <td>{puzzle.settings.nColors}/{puzzle.settings.containerVolume}/{formatEmptyContainers(puzzle.settings.nEmptyContainers)}</td>
                  <td>
                    <button id={`button-play-again-${puzzle.id}`} type="button" class="icon-with-label" onclick={() => playAgain(puzzle)}>
                      <svg viewBox="0 0 24 24" class="button-icon fill-icon"><use href="/icons/icon-play-seed.svg#icon-play-seed"/></svg>
                      <span>Play Again</span>
                    </button>
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
    padding: clamp(0.65rem, 2.4vw, 1rem);
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
  .icon-with-label {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    justify-content: center;
  }

  .board-wrap {
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: 10px;
    padding: clamp(0.65rem, 2.4vw, 1.5rem);
    overflow: hidden;
  }

  .board {
    --board-gap: 0.35rem;
    --row-gap: 0.7rem;
    --cell-gap: 2px;
    --cell-size: 24px;
    --container-side-pad: 2px;
    --container-bottom-pad: 2px;
    --container-top-pad: 0.82rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--row-gap);
  }

  .board.single-row-volume {
    --row-gap: 0;
  }

  .board-row {
    display: flex;
    justify-content: center;
    gap: var(--board-gap);
    width: min(100%, max-content);
  }

  .container {
    position: relative;
    display: grid;
    grid-auto-rows: auto;
    align-content: end;
    gap: var(--cell-gap);
    box-sizing: border-box;
    padding: var(--container-top-pad) var(--container-side-pad) var(--container-bottom-pad);
    border: 2px solid var(--line);
    border-radius: 0 0 8px 8px;
    width: calc(var(--cell-size) + (var(--container-side-pad) * 2) + 4px);
  }

  .container.selected {
    border-color: #ea4f4f;
  }

  .key-badge {
    position: absolute;
    top: -0.46rem;
    left: 50%;
    transform: translateX(-50%);
    font-size: clamp(0.44rem, calc(var(--cell-size) * 0.28), 0.62rem);
    border: 1px solid var(--line);
    border-radius: 999px;
    padding: 0.03rem 0.18rem;
    background: var(--surface);
  }

  .cell {
    display: block;
    width: var(--cell-size);
    aspect-ratio: 1 / 1;
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

  .action-icon-button .button-icon {
    width: 74%;
    height: 74%;
  }

  .action-icon-button:disabled {
    color: #8b92a3;
    border-color: #5f687b;
    background: #222a38;
    opacity: 0.68;
    cursor: not-allowed;
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
    .page {
      padding: 0.45rem;
      gap: 0.7rem;
    }

    .topbar {
      grid-template-columns: 1fr auto;
      justify-items: stretch;
    }

    #header-brand,
    .header-actions {
      grid-column: auto;
    }

    #header-brand {
      justify-self: start;
      text-align: left;
    }

    .header-actions {
      justify-self: end;
    }

    .status {
      align-items: flex-start;
    }

    .slider-label {
      align-items: flex-start;
      flex-direction: column;
    }

    .board-wrap {
      padding: 0.45rem;
    }
  }
</style>
