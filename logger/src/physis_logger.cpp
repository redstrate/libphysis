// SPDX-FileCopyrightText: 2023 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#include "physis_logger.h"

#include <QtLogging>
#include <QString>
#include <iostream>

extern "C" void set_tracing_callback(void (*callback) (QtMsgType type, const char*, const char*, int));

void callback(QtMsgType type, const char *message, const char *file, int line)
{
    QMessageLogContext context;
    context.file = file;
    context.line = line;
    context.category = "zone.xiv.physis";

    std::cout << qFormatLogMessage(type, context, QString::fromLocal8Bit(message)).toStdString() << std::endl;
}

void setup_physis_logging()
{
    set_tracing_callback(callback);
}