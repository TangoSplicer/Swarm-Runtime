import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

# 1. Add progress tracking to AppState
if "pub aggregations" in content and "pub progress" not in content:
    content = content.replace(
        "pub aggregations: Arc<DashMap<Uuid, Arc<Mutex<Vec<i32>>>>>,",
        "pub aggregations: Arc<DashMap<Uuid, Arc<Mutex<Vec<i32>>>>>, \n    pub progress: Arc<DashMap<Uuid, usize>>,"
    )

# 2. Initialize progress in main()
if "let aggregations = Arc::new(DashMap::new());" in content:
    content = content.replace(
        "let aggregations = Arc::new(DashMap::new());",
        "let aggregations = Arc::new(DashMap::new()); \n            let progress = Arc::new(DashMap::new());"
    )
    content = content.replace(
        "aggregations: aggregations.clone(),",
        "aggregations: aggregations.clone(), \n                progress: progress.clone(),"
    )

# 3. Update progress when a SHARD_RESULT is received
update_logic = """
                                                    if let Some(agg_ref) = agg_clone.get(&tid) {
                                                        agg_ref.value().lock().await.push(val);
                                                        // Update Progress Hook
                                                        progress_clone.entry(tid).and_modify(|e| *e += 1).or_insert(1);
                                                    }
"""
if "agg_ref.value().lock().await.push(val);" in content:
    import re
    content = re.sub(r"if let Some\(agg_ref\) = agg_clone\.get\(&tid\) \{.*?\}", update_logic, content, flags=re.DOTALL)

with open(path, "w") as f:
    f.write(content)
