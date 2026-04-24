const currency = new Intl.NumberFormat(undefined, {
  minimumFractionDigits: 2,
  maximumFractionDigits: 2,
});

const form = document.getElementById("transactionForm");
const formStatus = document.getElementById("formStatus");
const refreshButton = document.getElementById("refreshButton");
const clearFiltersButton = document.getElementById("clearFiltersButton");
const monthFilter = document.getElementById("monthFilter");
const categoryFilter = document.getElementById("categoryFilter");
const transactionRows = document.getElementById("transactionRows");

const incomeValue = document.getElementById("incomeValue");
const expenseValue = document.getElementById("expenseValue");
const netValue = document.getElementById("netValue");
const countValue = document.getElementById("countValue");

function currentFilters() {
  return {
    month: monthFilter.value,
    category: categoryFilter.value,
  };
}

function buildQuery(params) {
  const query = new URLSearchParams();

  if (params.month) {
    query.set("month", params.month);
  }

  if (params.category) {
    query.set("category", params.category);
  }

  const value = query.toString();
  return value ? `?${value}` : "";
}

async function requestJson(url, options = {}) {
  const response = await fetch(url, {
    headers: {
      "Content-Type": "application/json",
    },
    ...options,
  });

  const payload = await response.json().catch(() => ({ message: "Request failed." }));

  if (!response.ok) {
    throw new Error(payload.message || "Request failed.");
  }

  return payload;
}

function setSummary(summary) {
  incomeValue.textContent = currency.format(summary.income);
  expenseValue.textContent = currency.format(summary.expenses);
  netValue.textContent = currency.format(summary.net);
  countValue.textContent = String(summary.transaction_count);
}

function renderTransactions(transactions) {
  if (!transactions.length) {
    transactionRows.innerHTML = `
      <tr>
        <td colspan="7" class="empty-state">No transactions match the current filters.</td>
      </tr>
    `;
    return;
  }

  transactionRows.innerHTML = transactions
    .map(
      (transaction) => `
        <tr>
          <td>${transaction.id}</td>
          <td>${transaction.date}</td>
          <td>${transaction.transaction_type}</td>
          <td>${currency.format(transaction.amount)}</td>
          <td>${transaction.category}</td>
          <td>${transaction.description}</td>
          <td>
            <button class="delete-button" type="button" data-id="${transaction.id}">
              Delete
            </button>
          </td>
        </tr>
      `,
    )
    .join("");
}

async function loadDashboard() {
  const filters = currentFilters();
  const [transactions, summary] = await Promise.all([
    requestJson(`/api/transactions${buildQuery(filters)}`),
    requestJson(`/api/summary${buildQuery({ month: filters.month })}`),
  ]);

  renderTransactions(transactions);
  setSummary(summary);
}

form.addEventListener("submit", async (event) => {
  event.preventDefault();
  formStatus.textContent = "Saving...";

  const formData = new FormData(form);
  const payload = {
    transaction_type: formData.get("transaction_type"),
    amount: Number(formData.get("amount")),
    category: formData.get("category"),
    description: formData.get("description"),
    date: formData.get("date") || null,
  };

  try {
    await requestJson("/api/transactions", {
      method: "POST",
      body: JSON.stringify(payload),
    });
    form.reset();
    formStatus.textContent = "Transaction saved.";
    await loadDashboard();
  } catch (error) {
    formStatus.textContent = error.message;
  }
});

transactionRows.addEventListener("click", async (event) => {
  const target = event.target;
  if (!(target instanceof HTMLButtonElement) || !target.dataset.id) {
    return;
  }

  const shouldDelete = window.confirm(`Delete transaction ${target.dataset.id}?`);
  if (!shouldDelete) {
    return;
  }

  try {
    await requestJson(`/api/transactions/${target.dataset.id}`, {
      method: "DELETE",
    });
    await loadDashboard();
  } catch (error) {
    formStatus.textContent = error.message;
  }
});

refreshButton.addEventListener("click", () => {
  loadDashboard().catch((error) => {
    formStatus.textContent = error.message;
  });
});

clearFiltersButton.addEventListener("click", () => {
  monthFilter.value = "";
  categoryFilter.value = "";
  loadDashboard().catch((error) => {
    formStatus.textContent = error.message;
  });
});

monthFilter.addEventListener("change", () => {
  loadDashboard().catch((error) => {
    formStatus.textContent = error.message;
  });
});

categoryFilter.addEventListener("change", () => {
  loadDashboard().catch((error) => {
    formStatus.textContent = error.message;
  });
});

loadDashboard().catch((error) => {
  formStatus.textContent = error.message;
  transactionRows.innerHTML = `
    <tr>
      <td colspan="7" class="empty-state">${error.message}</td>
    </tr>
  `;
});
