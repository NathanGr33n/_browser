// CSS Animations and Transitions - Phase 7 Task 2

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Animation timing function (easing)
#[derive(Debug, Clone, PartialEq)]
pub enum TimingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f32, f32, f32, f32),
    Steps(u32, StepPosition),
}

/// Step position for steps() timing function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepPosition {
    Start,
    End,
}

impl TimingFunction {
    /// Calculate progress value at time t (0.0 to 1.0)
    pub fn calculate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        
        match self {
            TimingFunction::Linear => t,
            TimingFunction::Ease => Self::cubic_bezier(0.25, 0.1, 0.25, 1.0, t),
            TimingFunction::EaseIn => Self::cubic_bezier(0.42, 0.0, 1.0, 1.0, t),
            TimingFunction::EaseOut => Self::cubic_bezier(0.0, 0.0, 0.58, 1.0, t),
            TimingFunction::EaseInOut => Self::cubic_bezier(0.42, 0.0, 0.58, 1.0, t),
            TimingFunction::CubicBezier(x1, y1, x2, y2) => {
                Self::cubic_bezier(*x1, *y1, *x2, *y2, t)
            }
            TimingFunction::Steps(steps, position) => {
                let steps = (*steps).max(1) as f32;
                match position {
                    StepPosition::Start => (t * steps).ceil() / steps,
                    StepPosition::End => (t * steps).floor() / steps,
                }
            }
        }
    }
    
    /// Cubic bezier calculation (simplified Newton-Raphson method)
    fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> f32 {
        // Simplified: use approximation for performance
        // In production, would use proper Newton-Raphson iteration
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        
        3.0 * y1 * mt2 * t + 3.0 * y2 * mt * t2 + t3
    }
}

impl Default for TimingFunction {
    fn default() -> Self {
        TimingFunction::Ease
    }
}

/// CSS Animation property value that can be animated
#[derive(Debug, Clone, PartialEq)]
pub enum AnimatableValue {
    Number(f32),
    Color(u8, u8, u8, u8), // RGBA
    Length(f32), // pixels
    Percentage(f32),
    Transform(Transform),
}

/// Transform types for animations
#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub translate_x: f32,
    pub translate_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotate: f32, // degrees
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translate_x: 0.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotate: 0.0,
        }
    }
}

impl AnimatableValue {
    /// Interpolate between two values
    pub fn interpolate(&self, other: &Self, progress: f32) -> Option<Self> {
        let progress = progress.clamp(0.0, 1.0);
        
        match (self, other) {
            (AnimatableValue::Number(a), AnimatableValue::Number(b)) => {
                Some(AnimatableValue::Number(a + (b - a) * progress))
            }
            (AnimatableValue::Color(r1, g1, b1, a1), AnimatableValue::Color(r2, g2, b2, a2)) => {
                let r = (*r1 as f32 + (*r2 as i32 - *r1 as i32) as f32 * progress) as u8;
                let g = (*g1 as f32 + (*g2 as i32 - *g1 as i32) as f32 * progress) as u8;
                let b = (*b1 as f32 + (*b2 as i32 - *b1 as i32) as f32 * progress) as u8;
                let a = (*a1 as f32 + (*a2 as i32 - *a1 as i32) as f32 * progress) as u8;
                Some(AnimatableValue::Color(r, g, b, a))
            }
            (AnimatableValue::Length(a), AnimatableValue::Length(b)) => {
                Some(AnimatableValue::Length(a + (b - a) * progress))
            }
            (AnimatableValue::Percentage(a), AnimatableValue::Percentage(b)) => {
                Some(AnimatableValue::Percentage(a + (b - a) * progress))
            }
            (AnimatableValue::Transform(t1), AnimatableValue::Transform(t2)) => {
                Some(AnimatableValue::Transform(Transform {
                    translate_x: t1.translate_x + (t2.translate_x - t1.translate_x) * progress,
                    translate_y: t1.translate_y + (t2.translate_y - t1.translate_y) * progress,
                    scale_x: t1.scale_x + (t2.scale_x - t1.scale_x) * progress,
                    scale_y: t1.scale_y + (t2.scale_y - t1.scale_y) * progress,
                    rotate: t1.rotate + (t2.rotate - t1.rotate) * progress,
                }))
            }
            _ => None, // Type mismatch
        }
    }
}

/// A keyframe in an animation
#[derive(Debug, Clone)]
pub struct Keyframe {
    /// Offset (0.0 to 1.0)
    pub offset: f32,
    /// Property values at this keyframe
    pub values: HashMap<String, AnimatableValue>,
    /// Timing function for this segment
    pub timing_function: TimingFunction,
}

/// CSS @keyframes animation definition
#[derive(Debug, Clone)]
pub struct KeyframeAnimation {
    /// Animation name
    pub name: String,
    /// Keyframes sorted by offset
    pub keyframes: Vec<Keyframe>,
}

impl KeyframeAnimation {
    /// Create a new keyframe animation
    pub fn new(name: String) -> Self {
        Self {
            name,
            keyframes: Vec::new(),
        }
    }
    
    /// Add a keyframe (will be sorted by offset)
    pub fn add_keyframe(&mut self, keyframe: Keyframe) {
        self.keyframes.push(keyframe);
        self.keyframes.sort_by(|a, b| a.offset.partial_cmp(&b.offset).unwrap());
    }
    
    /// Get interpolated values at a given progress (0.0 to 1.0)
    pub fn get_values_at(&self, progress: f32) -> HashMap<String, AnimatableValue> {
        if self.keyframes.is_empty() {
            return HashMap::new();
        }
        
        let progress = progress.clamp(0.0, 1.0);
        
        // Find surrounding keyframes
        let mut before = &self.keyframes[0];
        let mut after = &self.keyframes[self.keyframes.len() - 1];
        
        for i in 0..self.keyframes.len() - 1 {
            if progress >= self.keyframes[i].offset && progress <= self.keyframes[i + 1].offset {
                before = &self.keyframes[i];
                after = &self.keyframes[i + 1];
                break;
            }
        }
        
        if before.offset == after.offset {
            return before.values.clone();
        }
        
        // Calculate local progress between keyframes
        let local_progress = (progress - before.offset) / (after.offset - before.offset);
        let eased_progress = before.timing_function.calculate(local_progress);
        
        // Interpolate values
        let mut result = HashMap::new();
        for (prop, before_val) in &before.values {
            if let Some(after_val) = after.values.get(prop) {
                if let Some(interpolated) = before_val.interpolate(after_val, eased_progress) {
                    result.insert(prop.clone(), interpolated);
                }
            }
        }
        
        result
    }
}

/// CSS Transition definition
#[derive(Debug, Clone)]
pub struct Transition {
    /// Property name to transition
    pub property: String,
    /// Duration of transition
    pub duration: Duration,
    /// Timing function
    pub timing_function: TimingFunction,
    /// Delay before starting
    pub delay: Duration,
}

/// Active animation instance
#[derive(Debug, Clone)]
pub struct ActiveAnimation {
    /// Animation name
    pub name: String,
    /// Duration
    pub duration: Duration,
    /// Start time
    pub start_time: Instant,
    /// Iteration count (0 = infinite)
    pub iteration_count: u32,
    /// Current iteration
    pub current_iteration: u32,
    /// Direction (normal or reverse)
    pub direction: AnimationDirection,
    /// Play state
    pub play_state: AnimationPlayState,
    /// Fill mode
    pub fill_mode: AnimationFillMode,
}

/// Animation direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

/// Animation play state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationPlayState {
    Running,
    Paused,
}

/// Animation fill mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

impl ActiveAnimation {
    /// Calculate current progress (0.0 to 1.0)
    pub fn current_progress(&self) -> f32 {
        if self.play_state == AnimationPlayState::Paused {
            return 0.0;
        }
        
        let elapsed = self.start_time.elapsed();
        let progress = elapsed.as_secs_f32() / self.duration.as_secs_f32();
        
        if progress >= 1.0 && self.iteration_count > 0 && self.current_iteration >= self.iteration_count {
            return 1.0;
        }
        
        let progress = progress % 1.0;
        
        match self.direction {
            AnimationDirection::Normal => progress,
            AnimationDirection::Reverse => 1.0 - progress,
            AnimationDirection::Alternate => {
                if self.current_iteration % 2 == 0 {
                    progress
                } else {
                    1.0 - progress
                }
            }
            AnimationDirection::AlternateReverse => {
                if self.current_iteration % 2 == 0 {
                    1.0 - progress
                } else {
                    progress
                }
            }
        }
    }
    
    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        if self.iteration_count == 0 {
            return false; // Infinite
        }
        
        let elapsed = self.start_time.elapsed();
        elapsed >= self.duration * self.iteration_count
    }
}

/// Animation manager
pub struct AnimationManager {
    /// Registered keyframe animations
    keyframe_animations: HashMap<String, KeyframeAnimation>,
    /// Active animations
    active_animations: Vec<ActiveAnimation>,
    /// Active transitions
    active_transitions: HashMap<String, (Instant, AnimatableValue, AnimatableValue, Transition)>,
}

impl AnimationManager {
    /// Create a new animation manager
    pub fn new() -> Self {
        Self {
            keyframe_animations: HashMap::new(),
            active_animations: Vec::new(),
            active_transitions: HashMap::new(),
        }
    }
    
    /// Register a keyframe animation
    pub fn register_keyframe_animation(&mut self, animation: KeyframeAnimation) {
        self.keyframe_animations.insert(animation.name.clone(), animation);
    }
    
    /// Start an animation
    pub fn start_animation(
        &mut self,
        name: String,
        duration: Duration,
        iteration_count: u32,
        direction: AnimationDirection,
        fill_mode: AnimationFillMode,
    ) -> bool {
        if !self.keyframe_animations.contains_key(&name) {
            return false;
        }
        
        self.active_animations.push(ActiveAnimation {
            name,
            duration,
            start_time: Instant::now(),
            iteration_count,
            current_iteration: 0,
            direction,
            play_state: AnimationPlayState::Running,
            fill_mode,
        });
        
        true
    }
    
    /// Start a transition
    pub fn start_transition(
        &mut self,
        property: String,
        from: AnimatableValue,
        to: AnimatableValue,
        transition: Transition,
    ) {
        self.active_transitions.insert(property, (Instant::now(), from, to, transition));
    }
    
    /// Update animations and get current values
    pub fn update(&mut self) -> HashMap<String, AnimatableValue> {
        let mut result = HashMap::new();
        
        // Update animations
        self.active_animations.retain_mut(|anim| {
            if anim.is_complete() {
                return false;
            }
            
            if let Some(keyframe_anim) = self.keyframe_animations.get(&anim.name) {
                let progress = anim.current_progress();
                let values = keyframe_anim.get_values_at(progress);
                
                for (prop, val) in values {
                    result.insert(prop, val);
                }
            }
            
            true
        });
        
        // Update transitions
        self.active_transitions.retain(|prop, (start_time, from, to, transition)| {
            let elapsed = start_time.elapsed();
            
            if elapsed < transition.delay {
                return true; // Not started yet
            }
            
            let progress = (elapsed - transition.delay).as_secs_f32() / transition.duration.as_secs_f32();
            
            if progress >= 1.0 {
                result.insert(prop.clone(), to.clone());
                return false; // Complete
            }
            
            let eased_progress = transition.timing_function.calculate(progress);
            if let Some(value) = from.interpolate(to, eased_progress) {
                result.insert(prop.clone(), value);
            }
            
            true
        });
        
        result
    }
    
    /// Check if any animations are active
    pub fn has_active_animations(&self) -> bool {
        !self.active_animations.is_empty() || !self.active_transitions.is_empty()
    }
    
    /// Pause an animation by name
    pub fn pause_animation(&mut self, name: &str) {
        for anim in &mut self.active_animations {
            if anim.name == name {
                anim.play_state = AnimationPlayState::Paused;
            }
        }
    }
    
    /// Resume an animation by name
    pub fn resume_animation(&mut self, name: &str) {
        for anim in &mut self.active_animations {
            if anim.name == name {
                anim.play_state = AnimationPlayState::Running;
            }
        }
    }
    
    /// Clear all animations
    pub fn clear(&mut self) {
        self.active_animations.clear();
        self.active_transitions.clear();
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_timing_function_linear() {
        let tf = TimingFunction::Linear;
        assert_eq!(tf.calculate(0.0), 0.0);
        assert_eq!(tf.calculate(0.5), 0.5);
        assert_eq!(tf.calculate(1.0), 1.0);
    }
    
    #[test]
    fn test_timing_function_steps() {
        let tf = TimingFunction::Steps(4, StepPosition::End);
        assert_eq!(tf.calculate(0.0), 0.0);
        assert_eq!(tf.calculate(0.3), 0.25);
        assert_eq!(tf.calculate(0.6), 0.5);
        assert_eq!(tf.calculate(1.0), 1.0);
    }
    
    #[test]
    fn test_animatable_value_interpolation() {
        let v1 = AnimatableValue::Number(0.0);
        let v2 = AnimatableValue::Number(100.0);
        
        assert_eq!(v1.interpolate(&v2, 0.0), Some(AnimatableValue::Number(0.0)));
        assert_eq!(v1.interpolate(&v2, 0.5), Some(AnimatableValue::Number(50.0)));
        assert_eq!(v1.interpolate(&v2, 1.0), Some(AnimatableValue::Number(100.0)));
    }
    
    #[test]
    fn test_color_interpolation() {
        let c1 = AnimatableValue::Color(0, 0, 0, 255);
        let c2 = AnimatableValue::Color(255, 255, 255, 255);
        
        if let Some(AnimatableValue::Color(r, g, b, a)) = c1.interpolate(&c2, 0.5) {
            assert_eq!(r, 127);
            assert_eq!(g, 127);
            assert_eq!(b, 127);
            assert_eq!(a, 255);
        } else {
            panic!("Expected color interpolation");
        }
    }
    
    #[test]
    fn test_keyframe_animation() {
        let mut anim = KeyframeAnimation::new("test".to_string());
        
        let mut kf1 = Keyframe {
            offset: 0.0,
            values: HashMap::new(),
            timing_function: TimingFunction::Linear,
        };
        kf1.values.insert("opacity".to_string(), AnimatableValue::Number(0.0));
        
        let mut kf2 = Keyframe {
            offset: 1.0,
            values: HashMap::new(),
            timing_function: TimingFunction::Linear,
        };
        kf2.values.insert("opacity".to_string(), AnimatableValue::Number(1.0));
        
        anim.add_keyframe(kf1);
        anim.add_keyframe(kf2);
        
        let values = anim.get_values_at(0.5);
        if let Some(AnimatableValue::Number(opacity)) = values.get("opacity") {
            assert!((opacity - 0.5).abs() < 0.01);
        } else {
            panic!("Expected opacity value");
        }
    }
    
    #[test]
    fn test_animation_manager() {
        let mut manager = AnimationManager::new();
        
        let mut anim = KeyframeAnimation::new("fade".to_string());
        let mut kf = Keyframe {
            offset: 0.0,
            values: HashMap::new(),
            timing_function: TimingFunction::Linear,
        };
        kf.values.insert("opacity".to_string(), AnimatableValue::Number(0.0));
        anim.add_keyframe(kf);
        
        manager.register_keyframe_animation(anim);
        
        assert!(manager.start_animation(
            "fade".to_string(),
            Duration::from_secs(1),
            1,
            AnimationDirection::Normal,
            AnimationFillMode::None,
        ));
        
        assert!(manager.has_active_animations());
    }
    
    #[test]
    fn test_transition() {
        let mut manager = AnimationManager::new();
        
        let transition = Transition {
            property: "width".to_string(),
            duration: Duration::from_millis(100),
            timing_function: TimingFunction::Linear,
            delay: Duration::ZERO,
        };
        
        manager.start_transition(
            "width".to_string(),
            AnimatableValue::Length(100.0),
            AnimatableValue::Length(200.0),
            transition,
        );
        
        assert!(manager.has_active_animations());
        
        // Update immediately should give start value
        let values = manager.update();
        if let Some(AnimatableValue::Length(width)) = values.get("width") {
            assert!(*width >= 100.0 && *width <= 200.0);
        }
    }
    
    #[test]
    fn test_transform_interpolation() {
        let t1 = AnimatableValue::Transform(Transform {
            translate_x: 0.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotate: 0.0,
        });
        
        let t2 = AnimatableValue::Transform(Transform {
            translate_x: 100.0,
            translate_y: 50.0,
            scale_x: 2.0,
            scale_y: 2.0,
            rotate: 90.0,
        });
        
        if let Some(AnimatableValue::Transform(t)) = t1.interpolate(&t2, 0.5) {
            assert!((t.translate_x - 50.0).abs() < 0.01);
            assert!((t.scale_x - 1.5).abs() < 0.01);
            assert!((t.rotate - 45.0).abs() < 0.01);
        } else {
            panic!("Expected transform interpolation");
        }
    }
}
