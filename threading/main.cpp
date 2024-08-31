#include <iostream>
#include <filesystem>
#include <fstream>
#include <thread>
#include <vector>
#include <mutex>
#include <queue>

std::mutex output_mutex;
std::mutex queue_mutex;
std::queue<std::filesystem::path> task_queue;
bool done = false;

void search_in_file(const std::filesystem::path& file_path, const std::string& search_term) {
    std::ifstream file(file_path);
    std::string line;
    size_t line_number = 0;

    while (std::getline(file, line)) {
        line_number++;
        if (line.find(search_term) != std::string::npos) {
            std::lock_guard<std::mutex> lock(output_mutex);
            std::cout << file_path << ":" << line_number << ": " << line << std::endl;
        }
    }
}

void worker_thread(const std::string& search_term) {
    while (true) {
        std::filesystem::path file_path;

        {
            std::lock_guard<std::mutex> lock(queue_mutex);
            if (!task_queue.empty()) {
                file_path = task_queue.front();
                task_queue.pop();
            } else if (done) {
                break;
            }
        }

        if (!file_path.empty()) {
            search_in_file(file_path, search_term);
        }
    }
}

void find_files(const std::filesystem::path& directory, const std::string& extension) {
    for (const auto& entry : std::filesystem::recursive_directory_iterator(directory)) {
        if (entry.path().extension() == extension) {
            std::lock_guard<std::mutex> lock(queue_mutex);
            task_queue.push(entry.path());
        }
    }
    done = true;
}

int main(int argc, char* argv[]) {
    if (argc < 3) {
        std::cerr << "Usage: " << argv[0] << " <file_extension> <search_term>\n";
        return 1;
    }

    std::string extension = argv[1];  // e.g., ".cpp"
    std::string search_term = argv[2]; // e.g., "BitState"
    std::filesystem::path directory = "."; // Start in the current directory

    // Start worker threads
    std::vector<std::thread> workers;
    for (unsigned int i = 0; i < std::thread::hardware_concurrency(); ++i) {
        workers.emplace_back(worker_thread, std::ref(search_term));
    }

    // Find files matching the extension
    find_files(directory, extension);

    // Join worker threads
    for (auto& worker : workers) {
        worker.join();
    }

    return 0;
}

