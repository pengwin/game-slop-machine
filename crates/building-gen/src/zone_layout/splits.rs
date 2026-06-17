pub fn distribute_splits(start: f32, end: f32, count: usize, tile_size: f32) -> Vec<f32> {
    if count <= 1 {
        return vec![start, end];
    }

    let total = end - start;
    let step = total / count as f32;
    let dir = if total >= 0.0 { 1.0 } else { -1.0 };

    let mut splits = Vec::with_capacity(count + 1);
    splits.push(start);

    for i in 1..count {
        let raw = start + i as f32 * step;
        splits.push(snap_to_grid(raw, tile_size));
    }

    splits.push(end);

    for i in 1..splits.len() - 1 {
        if dir > 0.0 {
            let min_pos = splits[i - 1] + tile_size;
            let max_pos = splits[i + 1] - tile_size;
            if splits[i] < min_pos {
                splits[i] = min_pos;
            }
            if splits[i] > max_pos {
                splits[i] = max_pos;
            }
        } else {
            let max_pos = splits[i - 1] - tile_size;
            let min_pos = splits[i + 1] + tile_size;
            if splits[i] > max_pos {
                splits[i] = max_pos;
            }
            if splits[i] < min_pos {
                splits[i] = min_pos;
            }
        }
    }

    splits
}

pub fn distribute_weighted_splits(start: f32, end: f32, weights: &[f32], tile_size: f32) -> Vec<f32> {
    if weights.len() <= 1 {
        return vec![start, end];
    }

    let total_weight: f32 = weights.iter().sum();
    if total_weight <= f32::EPSILON {
        return distribute_splits(start, end, weights.len(), tile_size);
    }

    let total = end - start;
    let mut splits = Vec::with_capacity(weights.len() + 1);
    let mut accumulated = 0.0;
    splits.push(start);

    for weight in weights.iter().take(weights.len() - 1) {
        accumulated += *weight;
        let raw = start + total * (accumulated / total_weight);
        splits.push(snap_to_grid(raw, tile_size));
    }

    splits.push(end);
    enforce_minimum_split_spacing(&mut splits, tile_size);
    splits
}

fn enforce_minimum_split_spacing(splits: &mut [f32], tile_size: f32) {
    if splits.len() <= 2 {
        return;
    }

    let dir = if splits[splits.len() - 1] >= splits[0] {
        1.0
    } else {
        -1.0
    };

    for i in 1..splits.len() - 1 {
        if dir > 0.0 {
            let min_pos = splits[i - 1] + tile_size;
            let max_pos = splits[i + 1] - tile_size;
            if splits[i] < min_pos {
                splits[i] = min_pos;
            }
            if splits[i] > max_pos {
                splits[i] = max_pos;
            }
        } else {
            let max_pos = splits[i - 1] - tile_size;
            let min_pos = splits[i + 1] + tile_size;
            if splits[i] > max_pos {
                splits[i] = max_pos;
            }
            if splits[i] < min_pos {
                splits[i] = min_pos;
            }
        }
    }
}

pub fn snap_to_grid(value: f32, tile_size: f32) -> f32 {
    (value / tile_size).round() * tile_size
}

pub fn sorted_pair(a: f32, b: f32) -> (f32, f32) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}
