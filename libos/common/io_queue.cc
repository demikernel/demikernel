#include "io_queue.hh"

#include <cerrno>

dmtr::io_queue::io_queue(enum category_id cid, int qd) :
    my_cid(cid),
    my_qd(qd)
{}

dmtr::io_queue::~io_queue()
{}

int dmtr::io_queue::socket(int domain, int type, int protocol) {
    return ENOTSUP;
}

int dmtr::io_queue::listen(int backlog) {
    return ENOTSUP;
}

int dmtr::io_queue::bind(const struct sockaddr * const saddr, socklen_t size) {
    return ENOTSUP;
}

int dmtr::io_queue::accept(io_queue *&q_out, struct sockaddr * const saddr_out, socklen_t * const size_out, int new_qd) {
    q_out = NULL;
    return ENOTSUP;
}

int dmtr::io_queue::connect(const struct sockaddr * const saddr, socklen_t size) {
    return ENOTSUP;
}

int dmtr::io_queue::close() {
    return 0;
}
