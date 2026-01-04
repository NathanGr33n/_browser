// Multi-Process Architecture - Phase 7 Task 6

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Process ID type
pub type ProcessId = u64;

/// Process type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcessType {
    /// Browser/main process
    Browser,
    /// Renderer process (per tab)
    Renderer,
    /// GPU process
    Gpu,
    /// Network service process
    Network,
}

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    /// Process is starting
    Starting,
    /// Process is running
    Running,
    /// Process has crashed
    Crashed,
    /// Process was terminated
    Terminated,
}

/// Inter-Process Communication message
#[derive(Debug, Clone)]
pub enum IpcMessage {
    /// Ping message
    Ping,
    /// Pong response
    Pong,
    /// Render frame request
    RenderFrame { width: u32, height: u32 },
    /// Render frame response
    RenderFrameResponse { frame_data: Vec<u8> },
    /// Navigate to URL
    Navigate { url: String },
    /// JavaScript evaluation request
    EvalScript { script: String },
    /// JavaScript evaluation response
    EvalScriptResponse { result: String },
    /// Process crash notification
    ProcessCrashed { process_id: ProcessId },
    /// Shutdown process
    Shutdown,
}

/// Process info
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    /// Process ID
    pub id: ProcessId,
    /// Process type
    pub process_type: ProcessType,
    /// Process state
    pub state: ProcessState,
    /// Parent process ID (if any)
    pub parent_id: Option<ProcessId>,
    /// Associated tab ID (for renderer processes)
    pub tab_id: Option<u64>,
}

/// Process handle
pub struct Process {
    /// Process info
    info: ProcessInfo,
    /// Message queue
    message_queue: Arc<Mutex<Vec<IpcMessage>>>,
}

impl Process {
    /// Create a new process
    pub fn new(id: ProcessId, process_type: ProcessType, parent_id: Option<ProcessId>) -> Self {
        Self {
            info: ProcessInfo {
                id,
                process_type,
                state: ProcessState::Starting,
                parent_id,
                tab_id: None,
            },
            message_queue: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// Get process ID
    pub fn id(&self) -> ProcessId {
        self.info.id
    }
    
    /// Get process type
    pub fn process_type(&self) -> ProcessType {
        self.info.process_type
    }
    
    /// Get state
    pub fn state(&self) -> ProcessState {
        self.info.state
    }
    
    /// Set state
    pub fn set_state(&mut self, state: ProcessState) {
        self.info.state = state;
    }
    
    /// Set tab ID
    pub fn set_tab_id(&mut self, tab_id: u64) {
        self.info.tab_id = Some(tab_id);
    }
    
    /// Send message to process
    pub fn send_message(&self, message: IpcMessage) -> Result<(), MultiprocessError> {
        let mut queue = self.message_queue.lock()
            .map_err(|_| MultiprocessError::LockError)?;
        
        // Enforce message queue limit
        if queue.len() >= 1000 {
            return Err(MultiprocessError::MessageQueueFull);
        }
        
        queue.push(message);
        Ok(())
    }
    
    /// Receive message from process
    pub fn receive_message(&self) -> Option<IpcMessage> {
        let mut queue = self.message_queue.lock().ok()?;
        if !queue.is_empty() {
            Some(queue.remove(0))
        } else {
            None
        }
    }
    
    /// Check message queue size
    pub fn message_queue_size(&self) -> usize {
        self.message_queue.lock().map(|q| q.len()).unwrap_or(0)
    }
}

/// Process manager
pub struct ProcessManager {
    /// Next process ID
    next_process_id: ProcessId,
    /// Active processes
    processes: HashMap<ProcessId, Process>,
    /// Process type counts
    type_counts: HashMap<ProcessType, usize>,
    /// Tab to renderer process mapping
    tab_to_process: HashMap<u64, ProcessId>,
}

impl ProcessManager {
    /// Create a new process manager
    pub fn new() -> Self {
        Self {
            next_process_id: 1,
            processes: HashMap::new(),
            type_counts: HashMap::new(),
            tab_to_process: HashMap::new(),
        }
    }
    
    /// Spawn a new process
    pub fn spawn_process(
        &mut self,
        process_type: ProcessType,
        parent_id: Option<ProcessId>,
    ) -> Result<ProcessId, MultiprocessError> {
        // Check process limits
        let type_count = self.type_counts.get(&process_type).unwrap_or(&0);
        if *type_count >= self.max_processes_for_type(process_type) {
            return Err(MultiprocessError::ProcessLimitReached);
        }
        
        let process_id = self.next_process_id;
        self.next_process_id += 1;
        
        let mut process = Process::new(process_id, process_type, parent_id);
        process.set_state(ProcessState::Running);
        
        self.processes.insert(process_id, process);
        *self.type_counts.entry(process_type).or_insert(0) += 1;
        
        Ok(process_id)
    }
    
    /// Get max processes for a type
    fn max_processes_for_type(&self, process_type: ProcessType) -> usize {
        match process_type {
            ProcessType::Browser => 1,
            ProcessType::Gpu => 1,
            ProcessType::Network => 1,
            ProcessType::Renderer => 100, // Per-tab isolation
        }
    }
    
    /// Spawn renderer process for tab
    pub fn spawn_renderer_for_tab(&mut self, tab_id: u64) -> Result<ProcessId, MultiprocessError> {
        let process_id = self.spawn_process(ProcessType::Renderer, Some(1))?;
        
        if let Some(process) = self.processes.get_mut(&process_id) {
            process.set_tab_id(tab_id);
        }
        
        self.tab_to_process.insert(tab_id, process_id);
        Ok(process_id)
    }
    
    /// Get renderer process for tab
    pub fn get_renderer_for_tab(&self, tab_id: u64) -> Option<ProcessId> {
        self.tab_to_process.get(&tab_id).copied()
    }
    
    /// Terminate process
    pub fn terminate_process(&mut self, process_id: ProcessId) -> Result<(), MultiprocessError> {
        let process = self.processes.get_mut(&process_id)
            .ok_or(MultiprocessError::ProcessNotFound)?;
        
        process.set_state(ProcessState::Terminated);
        
        // Cleanup tab mapping
        if let Some(tab_id) = process.info.tab_id {
            self.tab_to_process.remove(&tab_id);
        }
        
        // Decrement type count
        if let Some(count) = self.type_counts.get_mut(&process.process_type()) {
            *count = count.saturating_sub(1);
        }
        
        self.processes.remove(&process_id);
        Ok(())
    }
    
    /// Mark process as crashed
    pub fn mark_process_crashed(&mut self, process_id: ProcessId) -> Result<(), MultiprocessError> {
        let process = self.processes.get_mut(&process_id)
            .ok_or(MultiprocessError::ProcessNotFound)?;
        
        process.set_state(ProcessState::Crashed);
        Ok(())
    }
    
    /// Send IPC message
    pub fn send_ipc_message(
        &self,
        from_id: ProcessId,
        to_id: ProcessId,
        message: IpcMessage,
    ) -> Result<(), MultiprocessError> {
        // Verify sender exists
        if !self.processes.contains_key(&from_id) {
            return Err(MultiprocessError::ProcessNotFound);
        }
        
        let to_process = self.processes.get(&to_id)
            .ok_or(MultiprocessError::ProcessNotFound)?;
        
        to_process.send_message(message)
    }
    
    /// Receive IPC message
    pub fn receive_ipc_message(&self, process_id: ProcessId) -> Option<IpcMessage> {
        let process = self.processes.get(&process_id)?;
        process.receive_message()
    }
    
    /// Get process info
    pub fn get_process_info(&self, process_id: ProcessId) -> Option<ProcessInfo> {
        self.processes.get(&process_id).map(|p| p.info.clone())
    }
    
    /// Get all processes of type
    pub fn get_processes_by_type(&self, process_type: ProcessType) -> Vec<ProcessId> {
        self.processes
            .iter()
            .filter(|(_, p)| p.process_type() == process_type)
            .map(|(id, _)| *id)
            .collect()
    }
    
    /// Get total process count
    pub fn process_count(&self) -> usize {
        self.processes.len()
    }
    
    /// Clean up crashed processes
    pub fn cleanup_crashed_processes(&mut self) {
        let crashed: Vec<ProcessId> = self.processes
            .iter()
            .filter(|(_, p)| p.state() == ProcessState::Crashed)
            .map(|(id, _)| *id)
            .collect();
        
        for process_id in crashed {
            let _ = self.terminate_process(process_id);
        }
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared memory region (simplified)
pub struct SharedMemory {
    /// Region ID
    id: u64,
    /// Data
    data: Arc<Mutex<Vec<u8>>>,
    /// Size
    size: usize,
}

impl SharedMemory {
    /// Create a new shared memory region
    pub fn new(id: u64, size: usize) -> Self {
        Self {
            id,
            data: Arc::new(Mutex::new(vec![0; size])),
            size,
        }
    }
    
    /// Get region ID
    pub fn id(&self) -> u64 {
        self.id
    }
    
    /// Get size
    pub fn size(&self) -> usize {
        self.size
    }
    
    /// Write data
    pub fn write(&self, offset: usize, data: &[u8]) -> Result<(), MultiprocessError> {
        let mut mem = self.data.lock()
            .map_err(|_| MultiprocessError::LockError)?;
        
        if offset + data.len() > self.size {
            return Err(MultiprocessError::OutOfBounds);
        }
        
        mem[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }
    
    /// Read data
    pub fn read(&self, offset: usize, len: usize) -> Result<Vec<u8>, MultiprocessError> {
        let mem = self.data.lock()
            .map_err(|_| MultiprocessError::LockError)?;
        
        if offset + len > self.size {
            return Err(MultiprocessError::OutOfBounds);
        }
        
        Ok(mem[offset..offset + len].to_vec())
    }
}

/// Shared memory manager
pub struct SharedMemoryManager {
    /// Next region ID
    next_region_id: u64,
    /// Memory regions
    regions: HashMap<u64, SharedMemory>,
}

impl SharedMemoryManager {
    /// Create a new shared memory manager
    pub fn new() -> Self {
        Self {
            next_region_id: 1,
            regions: HashMap::new(),
        }
    }
    
    /// Create shared memory region
    pub fn create_region(&mut self, size: usize) -> Result<u64, MultiprocessError> {
        if size > 100 * 1024 * 1024 {  // 100MB limit
            return Err(MultiprocessError::MemoryLimitExceeded);
        }
        
        let region_id = self.next_region_id;
        self.next_region_id += 1;
        
        let region = SharedMemory::new(region_id, size);
        self.regions.insert(region_id, region);
        
        Ok(region_id)
    }
    
    /// Get shared memory region
    pub fn get_region(&self, region_id: u64) -> Option<&SharedMemory> {
        self.regions.get(&region_id)
    }
    
    /// Remove shared memory region
    pub fn remove_region(&mut self, region_id: u64) -> bool {
        self.regions.remove(&region_id).is_some()
    }
}

impl Default for SharedMemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Multi-process error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultiprocessError {
    /// Process not found
    ProcessNotFound,
    /// Process limit reached
    ProcessLimitReached,
    /// Message queue full
    MessageQueueFull,
    /// Lock error
    LockError,
    /// Memory limit exceeded
    MemoryLimitExceeded,
    /// Out of bounds access
    OutOfBounds,
    /// IPC error
    IpcError,
}

impl std::fmt::Display for MultiprocessError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MultiprocessError::ProcessNotFound => write!(f, "Process not found"),
            MultiprocessError::ProcessLimitReached => write!(f, "Process limit reached"),
            MultiprocessError::MessageQueueFull => write!(f, "Message queue full"),
            MultiprocessError::LockError => write!(f, "Lock error"),
            MultiprocessError::MemoryLimitExceeded => write!(f, "Memory limit exceeded"),
            MultiprocessError::OutOfBounds => write!(f, "Out of bounds access"),
            MultiprocessError::IpcError => write!(f, "IPC error"),
        }
    }
}

impl std::error::Error for MultiprocessError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_creation() {
        let process = Process::new(1, ProcessType::Renderer, None);
        assert_eq!(process.id(), 1);
        assert_eq!(process.process_type(), ProcessType::Renderer);
    }
    
    #[test]
    fn test_process_manager() {
        let mut manager = ProcessManager::new();
        let pid = manager.spawn_process(ProcessType::Renderer, None).unwrap();
        
        assert_eq!(manager.process_count(), 1);
        assert!(manager.get_process_info(pid).is_some());
    }
    
    #[test]
    fn test_spawn_renderer_for_tab() {
        let mut manager = ProcessManager::new();
        let tab_id = 42;
        
        let pid = manager.spawn_renderer_for_tab(tab_id).unwrap();
        assert_eq!(manager.get_renderer_for_tab(tab_id), Some(pid));
    }
    
    #[test]
    fn test_terminate_process() {
        let mut manager = ProcessManager::new();
        let pid = manager.spawn_process(ProcessType::Renderer, None).unwrap();
        
        manager.terminate_process(pid).unwrap();
        assert_eq!(manager.process_count(), 0);
    }
    
    #[test]
    fn test_ipc_message() {
        let mut manager = ProcessManager::new();
        let pid1 = manager.spawn_process(ProcessType::Browser, None).unwrap();
        let pid2 = manager.spawn_process(ProcessType::Renderer, None).unwrap();
        
        manager.send_ipc_message(pid1, pid2, IpcMessage::Ping).unwrap();
        
        let msg = manager.receive_ipc_message(pid2);
        assert!(matches!(msg, Some(IpcMessage::Ping)));
    }
    
    #[test]
    fn test_process_limit() {
        let mut manager = ProcessManager::new();
        
        // Try to spawn two browser processes (limit is 1)
        let _ = manager.spawn_process(ProcessType::Browser, None).unwrap();
        let result = manager.spawn_process(ProcessType::Browser, None);
        
        assert_eq!(result, Err(MultiprocessError::ProcessLimitReached));
    }
    
    #[test]
    fn test_shared_memory() {
        let mem = SharedMemory::new(1, 1024);
        
        let data = vec![1, 2, 3, 4, 5];
        mem.write(0, &data).unwrap();
        
        let read_data = mem.read(0, 5).unwrap();
        assert_eq!(read_data, data);
    }
    
    #[test]
    fn test_shared_memory_bounds() {
        let mem = SharedMemory::new(1, 10);
        
        let result = mem.write(8, &[1, 2, 3, 4]);
        assert_eq!(result, Err(MultiprocessError::OutOfBounds));
    }
    
    #[test]
    fn test_shared_memory_manager() {
        let mut manager = SharedMemoryManager::new();
        
        let region_id = manager.create_region(1024).unwrap();
        assert!(manager.get_region(region_id).is_some());
        
        assert!(manager.remove_region(region_id));
        assert!(manager.get_region(region_id).is_none());
    }
    
    #[test]
    fn test_process_crash_handling() {
        let mut manager = ProcessManager::new();
        let pid = manager.spawn_process(ProcessType::Renderer, None).unwrap();
        
        manager.mark_process_crashed(pid).unwrap();
        
        let info = manager.get_process_info(pid).unwrap();
        assert_eq!(info.state, ProcessState::Crashed);
        
        manager.cleanup_crashed_processes();
        assert_eq!(manager.process_count(), 0);
    }
    
    #[test]
    fn test_get_processes_by_type() {
        let mut manager = ProcessManager::new();
        
        manager.spawn_process(ProcessType::Renderer, None).unwrap();
        manager.spawn_process(ProcessType::Renderer, None).unwrap();
        manager.spawn_process(ProcessType::Browser, None).unwrap();
        
        let renderers = manager.get_processes_by_type(ProcessType::Renderer);
        assert_eq!(renderers.len(), 2);
    }
}
