const predefinedColors = [
  'red', 'blue', 'green', 'yellow', 'purple', 'orange', 'cyan', 'magenta', 'lime',
  'teal', 'pink', 'brown', 'navy', 'gray', 'black', 'white', 'gold', 'silver',
  'beige', 'coral', 'olive', 'maroon', 'indigo', 'violet', 'chocolate', 'tan',
  'aqua', 'salmon', 'khaki', 'amber', 'lavender', 'plum'
];

let itemTypes = [];
let containerCount;
let containerCapacity = 4; // Max items per container
let containers = [];
let selectedContainer = null;

function updateItemCount() {
  const count = document.getElementById('itemTypes').value;
  document.getElementById('itemCount').innerText = count;
}

function startGame() {
  const numItemTypes = parseInt(document.getElementById('itemTypes').value);
  if (isNaN(numItemTypes) || numItemTypes < 2 || numItemTypes > 32) {
    alert('Please select a valid number of item types (2-32).');
    return;
  }

  itemTypes = predefinedColors.slice(0, numItemTypes);
  containerCount = itemTypes.length + 2; // Number of item types + 2 empty containers
  containers = Array.from({length: containerCount}, () => []);

  // Initialize items with equal distribution
  const allItems = [];
  itemTypes.forEach(type => {
    for (let i = 0; i < containerCapacity; i++) {
      allItems.push(type);
    }
  });

  // Shuffle items
  for (let i = allItems.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [allItems[i], allItems[j]] = [allItems[j], allItems[i]];
  }

  // Distribute items to containers
  for (let i = 0; i < allItems.length; i++) {
    containers[i % (containerCount - 2)].push(allItems[i]);
  }

  renderGame();
}

function renderGame() {
  const gameDiv = document.getElementById('game');
  gameDiv.innerHTML = '';
  containers.forEach((container, index) => {
    const containerDiv = document.createElement('div');
    containerDiv.className = 'container';
    containerDiv.setAttribute('data-index', index);
    if (selectedContainer === index) {
      containerDiv.classList.add('selected');
    }

    // Render items from bottom to top
    container.forEach(item => {
      const itemDiv = document.createElement('div');
      itemDiv.className = 'item';
      itemDiv.style.backgroundColor = item;
      containerDiv.appendChild(itemDiv);
    });

    containerDiv.addEventListener('click', () => handleContainerClick(index));
    gameDiv.appendChild(containerDiv);
  });
}

function handleContainerClick(index) {
  if (selectedContainer === null) {
    // Select the first container
    selectedContainer = index;
  } else {
    // Move items to the new container
    moveItems(selectedContainer, index);
    selectedContainer = null;
  }
  renderGame();
}

function moveItems(fromIndex, toIndex) {
  const fromContainer = containers[fromIndex];
  const toContainer = containers[toIndex];

  if (!fromContainer.length) {
    alert('Source container is empty!');
    return;
  }

  if (toContainer.length >= containerCapacity) {
    alert('Destination container is full!');
    return;
  }

  const topItem = fromContainer[fromContainer.length - 1];
  const stack = [];

  // Get stack of matching items
  while (fromContainer.length && fromContainer[fromContainer.length - 1] === topItem) {
    stack.push(fromContainer.pop());
  }

  if (!toContainer.length || toContainer[toContainer.length - 1] === topItem) {
    toContainer.push(...stack);
  } else {
    fromContainer.push(...stack);
    alert('Items must match to be moved!');
  }

  if (checkWin()) {
    alert('You win!');
  }
}

function checkWin() {
  return containers.every(container => {
    return container.length === 0 || (container.length === containerCapacity && container.every(item => item === container[0]));
  });
}