/*
 * Copyright (C) the libgit2 contributors. All rights reserved.
 *
 * This file is part of libgit2, distributed under the GNU GPL v2 with
 * a Linking Exception. For full terms see the included COPYING file.
 */
#ifndef INCLUDE_streams_stransport_h__
#define INCLUDE_streams_stransport_h__

#include "common.h"

#include "git2/sys/stream.h"

extern int git_stransport_stream_new(git_stream **out, const char *host, const char *port);

#endif
