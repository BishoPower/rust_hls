/*
 * C Socket Library for Vivado DPI-C
 * =================================
 * TCP socket server implementation for SystemVerilog DPI
 * Compile with: gcc -shared -fPIC -o socket_lib.so socket_lib.c
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <sys/socket.h>
#include <sys/types.h>

static int server_fd = -1;
static int client_fd = -1;

/*
 * Initialize TCP socket server
 * Returns: socket file descriptor on success, -1 on error
 */
int socket_server_init(int port)
{
    struct sockaddr_in server_addr;
    int opt = 1;

    // Create socket
    server_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (server_fd < 0)
    {
        printf("❌ Socket creation failed\n");
        return -1;
    }

    // Set socket options
    if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt)) < 0)
    {
        printf("❌ Setsockopt failed\n");
        close(server_fd);
        return -1;
    }

    // Configure server address
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(port);

    // Bind socket
    if (bind(server_fd, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0)
    {
        printf("❌ Bind failed on port %d\n", port);
        close(server_fd);
        return -1;
    }

    // Listen for connections
    if (listen(server_fd, 1) < 0)
    {
        printf("❌ Listen failed\n");
        close(server_fd);
        return -1;
    }

    printf("🌐 Socket server listening on port %d\n", port);
    return server_fd;
}

/*
 * Accept client connection
 * Returns: client file descriptor on success, -1 on error
 */
int socket_accept_client()
{
    struct sockaddr_in client_addr;
    socklen_t client_len = sizeof(client_addr);

    printf("⏳ Waiting for client connection...\n");

    client_fd = accept(server_fd, (struct sockaddr *)&client_addr, &client_len);
    if (client_fd < 0)
    {
        printf("❌ Accept failed\n");
        return -1;
    }

    printf("✅ Client connected from %s:%d\n",
           inet_ntoa(client_addr.sin_addr),
           ntohs(client_addr.sin_port));

    return client_fd;
}

/*
 * Receive data from client
 * data: output array to store received integers
 * Returns: number of integers received, -1 on error
 */
int socket_receive_data(int *data)
{
    char buffer[1024];
    int bytes_received;

    if (client_fd < 0)
    {
        printf("❌ No client connected\n");
        return -1;
    }

    bytes_received = recv(client_fd, buffer, sizeof(buffer) - 1, 0);
    if (bytes_received <= 0)
    {
        if (bytes_received == 0)
        {
            printf("🔌 Client disconnected\n");
        }
        else
        {
            printf("❌ Receive error\n");
        }
        return -1;
    }

    buffer[bytes_received] = '\0';

    // Parse JSON-like format (simplified)
    // Expected: "bid_price,ask_price,bid_qty,ask_qty,bid_strong,ask_strong,position"
    int count = sscanf(buffer, "%d,%d,%d,%d,%d,%d,%d",
                       &data[0], &data[1], &data[2], &data[3],
                       &data[4], &data[5], &data[6]);

    if (count >= 7)
    {
        printf("📈 Received market data: Bid %d, Ask %d\n", data[0], data[1]);
        return count;
    }
    else
    {
        printf("❌ Invalid data format\n");
        return -1;
    }
}

/*
 * Send data to client
 * data: array of integers to send
 */
void socket_send_data(int *data)
{
    char buffer[256];
    int len;

    if (client_fd < 0)
    {
        printf("❌ No client connected\n");
        return;
    }

    // Format as CSV: action,price,quantity,ap_done,ap_idle,ap_ready
    len = snprintf(buffer, sizeof(buffer), "%d,%d,%d,%d,%d,%d\n",
                   data[0], data[1], data[2], data[3], data[4], data[5]);

    if (send(client_fd, buffer, len, 0) < 0)
    {
        printf("❌ Send failed\n");
    }
    else
    {
        printf("📤 Sent FPGA result: Action %d, Price %d, Qty %d\n",
               data[0], data[1], data[2]);
    }
}

/*
 * Close socket connections
 */
void socket_close()
{
    if (client_fd >= 0)
    {
        close(client_fd);
        client_fd = -1;
        printf("🔌 Client socket closed\n");
    }

    if (server_fd >= 0)
    {
        close(server_fd);
        server_fd = -1;
        printf("🔌 Server socket closed\n");
    }
}

// Test functions for debugging
void socket_test()
{
    printf("🧪 Socket library test - OK\n");
}
