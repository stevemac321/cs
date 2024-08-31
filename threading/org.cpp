#include <iostream>
#include <filesystem>
#include <map>
#include <thread>
#include <mutex>
#include <vector>
#include <system_error>

struct DirectoryInfo {
    size_t total_size;
    size_t file_count;
};

struct FileInfo {
    std::wstring filename;
    std::wstring path;
};

std::mutex mtx;
std::map<std::wstring, DirectoryInfo> directory_info_map;

void process_drive(const std::wstring& drive) {
    try {
        for (const auto& entry : std::filesystem::directory_iterator(drive)) {
            // Skip known problematic directories
            std::wstring dir_name = entry.path().filename().wstring();
            if (dir_name == L"System Volume Information" ||
                dir_name == L"Config.Msi" ||
                dir_name == L"Documents and Settings" ||
                dir_name == L"lost+found") {
                continue;  // Skip these directories
            }

            if (entry.is_directory()) {
                try {
                    size_t total_size = 0;
                    size_t file_count = 0;

                    for (const auto& file : std::filesystem::directory_iterator(entry.path())) {
                        if (file.is_regular_file()) {
                            total_size += std::filesystem::file_size(file);
                            file_count++;
                        }
                    }

                    // Use the directory path as the key
                    std::wstring path = entry.path().wstring();
                    DirectoryInfo dir_info = { total_size, file_count };

                    std::lock_guard<std::mutex> lock(mtx);
                    directory_info_map[path] = dir_info;

                }
                catch (const std::filesystem::filesystem_error& e) {
                    std::wcerr << L"Error accessing directory "
                        << entry.path().wstring() << L": "
                        << e.what() << '\n';
                }
            }
        }
    }
    catch (const std::filesystem::filesystem_error& e) {
        std::wcerr << L"Error accessing drive " << drive << L": "
            << e.what() << '\n';
    }
}

void process_files(const std::vector<std::wstring>& directories, std::map<std::wstring, std::wstring>& file_map) {
    std::vector<std::wstring> extensions = {
        L".c", L".cpp", L".h", L".hpp", L".inl", L".rs", L".s", L".asm", L".S",
        L".sql", L".html", L".docx", L".pdf", L".py"
    };

    for (const auto& dir : directories) {
        try {
            for (const auto& entry : std::filesystem::recursive_directory_iterator(dir, std::filesystem::directory_options::skip_permission_denied)) {
                try {
                    if (entry.is_regular_file()) {
                        std::wstring extension = entry.path().extension().wstring();

                        if (extension == L".exe") {
                            continue; // Skip .exe files
                        }

                        if (std::find(extensions.begin(), extensions.end(), extension) != extensions.end()) {
                            std::lock_guard<std::mutex> lock(mtx);
                            file_map[entry.path().filename().wstring()] = entry.path().wstring();
                        }
                    }
                }
                catch (const std::filesystem::filesystem_error&) {
                    // Exception caught, but no logging or printing
                    continue;  // Silently skip to the next file
                }
            }
        }
        catch (const std::filesystem::filesystem_error&) {
            // Exception caught, but no logging or printing
            continue;  // Silently skip to the next directory
        }
    }
}


int main() {
    std::vector<std::wstring> drives = { L"C:\\", L"D:\\", L"E:\\", L"U:\\", L"V:\\", L"W:\\", L"X:\\", L"Y:\\" };
    std::vector<std::thread> threads;

    for (const auto& drive : drives) {
        threads.emplace_back(process_drive, drive);
    }

    for (auto& t : threads) {
        t.join();
    }

    // Convert map to vector for sorting
    std::vector<std::pair<std::wstring, DirectoryInfo>> dir_vector;
    for (const auto& entry : directory_info_map) {
        if (entry.second.total_size > 0 && entry.second.file_count > 0) {
            dir_vector.push_back(entry);
        }
    }

    // Sort the vector by DirectoryInfo::total_size
    std::sort(dir_vector.begin(), dir_vector.end(),
        [](const std::pair<std::wstring, DirectoryInfo>& a, const std::pair<std::wstring, DirectoryInfo>& b) {
            return a.second.total_size > b.second.total_size; // Sort in descending order
        });

    // Create 16 vectors for distribution
    const int num_threads = 16;
    std::vector<std::vector<std::wstring>> thread_vectors(num_threads);

    // Distribute directories to threads in a round-robin fashion
    for (size_t i = 0; i < dir_vector.size(); ++i) {
        thread_vectors[i % num_threads].push_back(dir_vector[i].first);
    }

    // Create a map to store file info for each thread
    std::vector<std::map<std::wstring, std::wstring>> file_maps(num_threads);

    // Create and launch threads
    threads.clear();  // Reuse the same vector for thread storage
    for (int i = 0; i < num_threads; ++i) {
        threads.emplace_back(process_files, thread_vectors[i], std::ref(file_maps[i]));
    }

    // Join threads
    for (auto& t : threads) {
        t.join();
    }

    // Optionally, combine results from all threads or process them individually
    std::map<std::wstring, std::wstring> combined_file_map;
    for (const auto& file_map : file_maps) {
        combined_file_map.insert(file_map.begin(), file_map.end());
    }

    // Example of processing or printing out the combined file map
    for (const auto& [filename, path] : combined_file_map) {
        std::wcout << L"File: " << filename << L" Path: " << path << '\n';
    }

    return 0;
}